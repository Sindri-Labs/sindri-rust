use crate::utils::{compress_directory, SINDRI_IGNORE_FILENAME, SINDRI_MANIFEST_FILENAME};
use crate::client::SindriClient;

use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;
use std::path::PathBuf;
use std::io::Cursor;
use tar::Archive;

fn create_test_directory() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();
    
    // Create a valid sindri.json
    let manifest_content = r#"{"name": "test-circuit", "circuitType": "circom"}"#;
    let manifest_path = dir_path.join(SINDRI_MANIFEST_FILENAME);
    let mut file = File::create(manifest_path).unwrap();
    file.write_all(manifest_content.as_bytes()).unwrap();

    // Create some test files
    let test_file_path = dir_path.join("some_artifact.circom");
    let mut file = File::create(test_file_path).unwrap();
    file.write_all(b"test content").unwrap();

    (temp_dir, dir_path)
}

#[tokio::test]
async fn test_successful_compression() {
    let (_temp_dir, dir_path) = create_test_directory();
    
    let result = compress_directory(&dir_path, None).await;
    assert!(result.is_ok());
    
    let compressed_data = result.unwrap();
    assert!(!compressed_data.is_empty());
}

#[tokio::test]
async fn test_missing_manifest() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    let result = compress_directory(&dir_path, None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[tokio::test]
async fn test_invalid_json_manifest() {
    let (_temp_dir, dir_path) = create_test_directory();
    
    // Overwrite with invalid JSON
    let manifest_path = dir_path.join(SINDRI_MANIFEST_FILENAME);
    fs::write(manifest_path, "nonjson").unwrap();

    let result = compress_directory(&dir_path, None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
}

#[tokio::test]
async fn test_sindriignore_respected() {
    let (_temp_dir, dir_path) = create_test_directory();
    
    // Create .sindriignore and a file to be ignored
    let ignore_content = "ignored.txt";
    fs::write(dir_path.join(SINDRI_IGNORE_FILENAME), ignore_content).unwrap();
    fs::write(dir_path.join("ignored.txt"), "should be ignored").unwrap();
    
    let circuit = compress_directory(&dir_path, None).await;
    assert!(circuit.is_ok());
    
    let cursor = Cursor::new(circuit.unwrap());
    let gz_decoder = GzDecoder::new(cursor);
    let mut archive = Archive::new(gz_decoder);

    let file_names: Vec<String> = archive.entries().unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.path().ok().map(|p| p.to_string_lossy().into_owned()))
        .collect();

    assert!(!file_names.contains(&"ignored.txt".to_string()));
}


#[tokio::test]
async fn test_hidden_files_ignored() {
    let (_temp_dir, dir_path) = create_test_directory();
    
    fs::write(dir_path.join(".hidden"), "hidden content").unwrap();
    
    let circuit = compress_directory(&dir_path, None).await;
    assert!(circuit.is_ok());
    
    let cursor = Cursor::new(circuit.unwrap());
    let gz_decoder = GzDecoder::new(cursor);
    let mut archive = Archive::new(gz_decoder);

    let file_names: Vec<String> = archive.entries().unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.path().ok().map(|p| p.to_string_lossy().into_owned()))
        .collect();

    assert!(!file_names.contains(&".hidden".to_string()));
}

#[tokio::test]
async fn test_max_project_size_exceeded() {
    let (_temp_dir, dir_path) = create_test_directory();
    
    // Create a file that's intentionally too large
    let test_file_path = dir_path.join("large_file.txt");
    let content: String = (0..1000)  
        .map(|_| rand::random::<u8>() as char)
        .collect();
    fs::write(test_file_path, content).unwrap();
    
    // Set max size to 100 bytes
    let result = compress_directory(&dir_path, Some(100)).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("project directory exceeds"));
}

#[tokio::test]
async fn test_create_circuit_invalid_file() {
    let (_temp_dir, dir_path) = create_test_directory();
    let test_file_path = dir_path.join("some_artifact.circom");

    let client = SindriClient::new(None);
    let result = client.create_circuit(test_file_path.to_string_lossy().to_string(), None, None).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not a zip file or tarball"));
}

#[tokio::test]
async fn test_create_circuit_nonexistent_path() {
    let client = SindriClient::new(None);
    let result = client.create_circuit("nonexistent/path".to_string(), None, None).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not a file or directory"));
}

