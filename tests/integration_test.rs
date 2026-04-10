use std::process::Command;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_fox_smile() {
    let pixaki_path = PathBuf::from("tests/data/fox_smile.pixaki");
    let output_path = PathBuf::from("tests/data/fox_smile.aseprite");
    
    // Ensure output doesn't exist
    if output_path.exists() {
        fs::remove_file(&output_path).unwrap();
    }

    let status = Command::new("cargo")
        .args(["run", "--", pixaki_path.to_str().unwrap(), output_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute command");

    assert!(status.success());
    assert!(output_path.exists());
    
    // Optional: Clean up
    fs::remove_file(&output_path).unwrap();
}

#[test]
fn test_fox_walk_2010s() {
    let pixaki_path = PathBuf::from("tests/data/fox_walk.pixaki");
    let output_path = PathBuf::from("tests/data/fox_walk.aseprite");
    
    // Ensure output doesn't exist
    if output_path.exists() {
        fs::remove_file(&output_path).unwrap();
    }

    let status = Command::new("cargo")
        .args(["run", "--", pixaki_path.to_str().unwrap(), output_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute command");

    // This will likely fail for now as it's an old format
    assert!(status.success());
    assert!(output_path.exists());
    
    // Optional: Clean up
    fs::remove_file(&output_path).unwrap();
}
