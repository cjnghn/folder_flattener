use anyhow::{anyhow, Context, Result};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn flatten_directory(source: &str, output: &str, ignore_file: Option<&str>) -> Result<()> {
    let source_path = Path::new(source);
    if !source_path.exists() {
        return Err(anyhow!("Source directory does not exist: {}", source));
    }
    if !source_path.is_dir() {
        return Err(anyhow!("Source path is not a directory: {}", source));
    }

    fs::create_dir_all(output).context("Failed to create output directory")?;

    let mut builder = WalkBuilder::new(source);
    builder.hidden(false); // Include hidden files and directories

    // Add custom ignore file if provided
    if let Some(ignore_path) = ignore_file {
        builder.add_ignore(ignore_path);
    }

    // Add .flattenerignore file if it exists
    let flattenerignore_path = source_path.join(".flattenerignore");
    if flattenerignore_path.exists() {
        builder.add_ignore(&flattenerignore_path);
    }

    let walker = builder.build();
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );

    for entry in walker
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map_or(false, |ft| ft.is_file()))
    {
        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(source).unwrap();
        let output_path = Path::new(output).join(relative_path.file_name().unwrap());

        pb.set_message(format!("Processing: {}", relative_path.display()));

        let mut output_file = File::create(&output_path)
            .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

        // Add comment with file path based on file type
        let comment =
            get_comment_for_file(relative_path, relative_path.to_str().unwrap().to_string());
        writeln!(output_file, "{}", comment)?;

        let mut source_file = File::open(source_path)
            .with_context(|| format!("Failed to open source file: {}", source_path.display()))?;
        io::copy(&mut source_file, &mut output_file)
            .with_context(|| format!("Failed to copy file content: {}", source_path.display()))?;

        info!(
            "Processed: {} -> {}",
            relative_path.display(),
            output_path.display()
        );
    }

    pb.finish_with_message("Flattening complete");
    Ok(())
}

fn get_comment_for_file(path: &Path, file_path: String) -> String {
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    match extension {
        "py" => format!("# path: {}", file_path),
        "js" | "ts" | "css" | "scss" | "sass" | "less" => format!("// path: {}", file_path),
        "html" | "xml" => format!("<!-- path: {} -->", file_path),
        "rs" => format!("// path: {}", file_path),
        "rb" => format!("# path: {}", file_path),
        "php" => format!("<?php // path: {} ?>", file_path),
        "lua" => format!("-- path: {}", file_path),
        "sql" => format!("-- path: {}", file_path),
        "hs" | "lhs" => format!("-- path: {}", file_path),
        "m" | "mm" => format!("// path: {}", file_path),
        "swift" => format!("// path: {}", file_path),
        "go" => format!("// path: {}", file_path),
        "java" | "c" | "cpp" | "cs" => format!("// path: {}", file_path),
        "sh" | "bash" => format!("# path: {}", file_path),
        "txt" => format!("// path: {}", file_path), // Added for .txt files
        _ => format!("# path: {}", file_path),      // Default to '#' for unknown file types
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_get_comment_for_file() {
        let path = Path::new("test.rs");
        assert_eq!(
            get_comment_for_file(path, "test.rs".to_string()),
            "// path: test.rs"
        );

        let path = Path::new("test.py");
        assert_eq!(
            get_comment_for_file(path, "test.py".to_string()),
            "# path: test.py"
        );

        let path = Path::new("test.unknown");
        assert_eq!(
            get_comment_for_file(path, "test.unknown".to_string()),
            "# path: test.unknown"
        );
    }

    #[test]
    fn test_flatten_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let output_dir = temp_dir.path().join("output");

        fs::create_dir_all(source_dir.join("subdir"))?;
        File::create(source_dir.join("file1.txt"))?.write_all(b"content1")?;
        File::create(source_dir.join("subdir/file2.txt"))?.write_all(b"content2")?;

        // Create .flattenerignore file
        File::create(source_dir.join(".flattenerignore"))?.write_all(b"*.log\n")?;

        flatten_directory(
            source_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
            None,
        )?;

        assert!(output_dir.join("file1.txt").exists());
        assert!(output_dir.join("file2.txt").exists());

        let content1 = fs::read_to_string(output_dir.join("file1.txt"))?;
        let content2 = fs::read_to_string(output_dir.join("file2.txt"))?;

        assert!(content1.contains("// path: file1.txt"));
        assert!(content1.contains("content1"));
        assert!(content2.contains("// path: subdir/file2.txt"));
        assert!(content2.contains("content2"));

        // Test ignore functionality
        File::create(source_dir.join("ignored.log"))?.write_all(b"This should be ignored")?;
        assert!(!output_dir.join("ignored.log").exists());

        Ok(())
    }

    #[test]
    fn test_flatten_directory_nonexistent_source() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let nonexistent_dir = temp_dir.path().join("nonexistent");
        let output_dir = temp_dir.path().join("output");

        let result = flatten_directory(
            nonexistent_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
            None,
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Source directory does not exist"));

        Ok(())
    }
}
