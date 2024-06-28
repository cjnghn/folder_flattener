use std::fs::{self, File};
use std::io::{self, Write, BufRead};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use ignore::{WalkBuilder, overrides::OverrideBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};

pub fn flatten_directory(source: &str, output: &str, ignore_file: Option<&str>) -> Result<()> {
    fs::create_dir_all(output).context("Failed to create output directory")?;

    let mut builder = WalkBuilder::new(source);
    
    // Add .flatternignore file if it exists
    let flatternignore_path = Path::new(source).join(".flatternignore");
    if flatternignore_path.exists() {
        let override_builder = build_override(&flatternignore_path)?;
        builder.overrides(override_builder.build()?);
    }

    // Add user-specified ignore file if provided
    if let Some(ignore_path) = ignore_file {
        builder.add_ignore(ignore_path);
    }

    let walker = builder.build();
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} [{elapsed_precise}] {msg}")
        .unwrap());

    for entry in walker.filter_map(Result::ok).filter(|e| e.file_type().map_or(false, |ft| ft.is_file())) {
        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(source).unwrap();
        let output_path = Path::new(output).join(relative_path.file_name().unwrap());

        pb.set_message(format!("Processing: {}", relative_path.display()));

        let mut output_file = File::create(&output_path)
            .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;
        
        // Add comment with file path based on file type
        let comment = get_comment_for_file(&relative_path, relative_path.display().to_string());
        writeln!(output_file, "{}", comment)?;

        let mut source_file = File::open(source_path)
            .with_context(|| format!("Failed to open source file: {}", source_path.display()))?;
        io::copy(&mut source_file, &mut output_file)
            .with_context(|| format!("Failed to copy file content: {}", source_path.display()))?;

        info!("Processed: {} -> {}", relative_path.display(), output_path.display());
    }

    pb.finish_with_message("Flattening complete");
    Ok(())
}

fn build_override(flatternignore_path: &Path) -> Result<OverrideBuilder> {
    let file = File::open(flatternignore_path)
        .with_context(|| format!("Failed to open .flatternignore file: {}", flatternignore_path.display()))?;
    let reader = io::BufReader::new(file);
    let mut builder = OverrideBuilder::new(flatternignore_path.parent().unwrap());

    for line in reader.lines() {
        let line = line.context("Failed to read line from .flatternignore")?;
        let line = line.trim();
        if !line.is_empty() && !line.starts_with('#') {
            builder.add(&line).context("Failed to add ignore pattern")?;
        }
    }

    Ok(builder)
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
        _ => format!("# path: {}", file_path),  // Default to '#' for unknown file types
    }
}