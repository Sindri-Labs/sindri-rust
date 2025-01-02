use std::{fs::File, io::Write, path::PathBuf};

use tempfile::TempDir;

use sindri_rs::client::SindriClient;

fn create_test_circuit() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    // Create a sindri.json
    let sindri_manifest = r#"{"name": "test-circuit", "circuitType": "circom"}"#;
    let manifest_path = dir_path.join("sindri.json");
    let mut file = File::create(manifest_path).unwrap();
    file.write_all(sindri_manifest.as_bytes()).unwrap();

    // Create a baby circuit
    let circom_circuit = r#"pragma circom 2.0.0;
template Multiplier2 () {  
   signal input a;  
   signal input b;  
   signal output c;  
   c <== a * b;  
}"#;
    let test_file_path = dir_path.join("circuit.circom");
    let mut file = File::create(test_file_path).unwrap();
    file.write_all(circom_circuit.as_bytes()).unwrap();

    (temp_dir, dir_path)
}

#[tokio::test]
async fn test_create_circuit() {
    let (_temp_dir, dir_path) = create_test_circuit();

    let client = SindriClient::new(None);
    let result = client
        .create_circuit(dir_path.to_string_lossy().to_string(), None, None)
        .await;

    assert!(result.is_ok());
}
