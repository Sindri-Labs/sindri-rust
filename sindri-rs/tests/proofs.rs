use sindri_rs::client::SindriClient;
use std::time::Duration;
use tokio::time::sleep;

mod factory;

// THIS WONT WORK TIL TAGS ARE IMPLEMENTED

// #[tokio::test]
// async fn test_create_proof() {

//     let (_temp_dir, dir_path) = factory::baby_circuit();

//     let client = SindriClient::new(None);

//     // Due to parallel execution of tests, the circuit with tag "latest"
//     // may not exist or be ready yet
//     let circuit = client
//         .create_circuit(dir_path.to_string_lossy().to_string(), None, None)
//         .await;

//     let input = r#"{"a": 1, "b": 2}"#;
//     let result = client.prove_circuit("test-circuit", input, None, None).await;

//     assert!(result.is_ok());
//     if let Ok(proof) = result {
//         assert!(!proof.proof_id.is_empty());
//     }
// }
