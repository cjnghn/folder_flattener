use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_flatten_directory_success() {   // 테스트: 폴더 평탄화 성공
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");

    // Create source directory structure
    fs::create_dir_all(source_dir.join("subdir")).unwrap();
    File::create(source_dir.join("file1.txt"))
        .unwrap()
        .write_all(b"content1")
        .unwrap();
    File::create(source_dir.join("subdir/file2.txt"))
        .unwrap()
        .write_all(b"content2")
        .unwrap();

    let mut cmd = Command::cargo_bin("folder-flattener").unwrap();
    cmd.arg("-s").arg(source_dir).arg("-o").arg(&output_dir);

    cmd.assert().success();

    // Check if files are flattened correctly
    assert!(output_dir.join("file1.txt").exists());
    assert!(output_dir.join("file2.txt").exists());

    // Check if file contents are preserved
    assert_eq!(
        fs::read_to_string(output_dir.join("file1.txt")).unwrap(),
        "// path: file1.txt\ncontent1"
    );
    assert_eq!(
        fs::read_to_string(output_dir.join("file2.txt")).unwrap(),
        "// path: subdir/file2.txt\ncontent2"
    );
}

#[test]
fn test_flatten_directory_with_ignore() {   // 테스트: 폴더 평탄화 무시 (.flattenerignore 파일 사용)
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");

    // Create source directory structure
    fs::create_dir_all(source_dir.join("subdir")).unwrap();
    File::create(source_dir.join("file1.txt"))
        .unwrap()
        .write_all(b"content1")
        .unwrap();
    File::create(source_dir.join("subdir/file2.txt"))
        .unwrap()
        .write_all(b"content2")
        .unwrap();
    File::create(source_dir.join("ignored.txt"))
        .unwrap()
        .write_all(b"ignored")
        .unwrap();

    // Create .flattenerignore file
    File::create(source_dir.join(".flattenerignore"))
        .unwrap()
        .write_all(b"ignored.txt")
        .unwrap();

    let mut cmd = Command::cargo_bin("folder-flattener").unwrap();
    cmd.arg("-s").arg(source_dir).arg("-o").arg(&output_dir);

    cmd.assert().success();

    // Check if files are flattened correctly and ignored file is not present
    assert!(output_dir.join("file1.txt").exists());
    assert!(output_dir.join("file2.txt").exists());
    assert!(!output_dir.join("ignored.txt").exists());
}

#[test]
fn test_flatten_directory_nonexistent_source() { // 테스트: 존재하지 않는 소스 디렉토리
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("nonexistent");
    let output_dir = temp_dir.path().join("output");

    let mut cmd = Command::cargo_bin("folder-flattener").unwrap();
    cmd.arg("-s").arg(source_dir).arg("-o").arg(&output_dir);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Source directory does not exist"));
}

#[test]
fn test_flatten_directory_output_exists() { // 테스트: 출력 디렉토리가 이미 존재하는 경우
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir_all(&source_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    File::create(output_dir.join("existing_file.txt"))
        .unwrap()
        .write_all(b"existing")
        .unwrap();

    let mut cmd = Command::cargo_bin("folder-flattener").unwrap();
    cmd.arg("-s").arg(source_dir).arg("-o").arg(&output_dir);

    cmd.assert().success();

    // Check if existing file is still there
    assert!(output_dir.join("existing_file.txt").exists());
    assert_eq!(
        fs::read_to_string(output_dir.join("existing_file.txt")).unwrap(),
        "existing"
    );
}
