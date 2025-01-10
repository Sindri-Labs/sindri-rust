use std::collections::HashMap;

use serde_json::json;

use sindri_rs::{client::SindriClient, CircuitInfo, JobStatus};

mod factory;

#[tokio::test]
async fn test_create_proof_basic() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None, None);

    // Due to parallel execution of tests, the circuit with tag "latest" may not exist or be ready yet
    // We use tags to distinguish tests run on the same circuit
    let circuit = client
        .create_circuit(
            dir_path.to_string_lossy().to_string(),
            Some(vec!["prove_basic".to_string()]),
            None,
        )
        .await;

    assert!(circuit.is_ok());
    let circuit = circuit.unwrap();
    let circuit_identifier = circuit.id();

    let input = r#"{"a": 1, "b": 2}"#;
    let test_meta = HashMap::from([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]);
    let result = client
        .prove_circuit(
            circuit_identifier,
            input,
            Some(test_meta.clone()),
            None,
            None,
        )
        .await;

    assert!(result.is_ok());
    let proof = result.unwrap();

    assert!(!proof.proof_id.is_empty());
    assert_eq!(proof.meta, test_meta);
}

#[tokio::test]
async fn test_create_proof_input_modes() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None, None);

    // Due to parallel execution of tests, the circuit with tag "latest" may not exist or be ready yet
    // We use tags to distinguish tests run on the same circuit
    let circuit = client
        .create_circuit(
            dir_path.to_string_lossy().to_string(),
            Some(vec!["input_modes".to_string()]),
            None,
        )
        .await;

    assert!(circuit.is_ok());
    let circuit = circuit.unwrap();
    let circuit_identifier = circuit.id();

    let str_input = r#"{"a": 1, "b": 2}"#;
    let str_submit = client.prove_circuit(circuit_identifier, str_input, None, None, None);

    let json_input = json!({"a": 1, "b": 2});
    let json_submit = client.prove_circuit(circuit_identifier, json_input, None, None, None);

    let owned_input = String::from(str_input);
    let owned_submit = client.prove_circuit(circuit_identifier, owned_input, None, None, None);

    let (str_result, json_result, owned_result) =
        tokio::try_join!(str_submit, json_submit, owned_submit).unwrap();

    assert_eq!(str_result.status, JobStatus::Ready);
    assert_eq!(json_result.status, JobStatus::Ready);
    assert_eq!(owned_result.status, JobStatus::Ready);

    assert_eq!(str_result.public, json_result.public);
    assert_eq!(json_result.public, owned_result.public);
    assert_eq!(str_result.public, owned_result.public);
}

#[tokio::test]
async fn test_delete_proof() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None, None);

    let result = client
        .create_circuit(
            dir_path.to_string_lossy().to_string(),
            Some(vec!["proof_deletion".to_string()]),
            None,
        )
        .await;

    assert!(result.is_ok());
    let circuit = result.unwrap();

    let input = json!({"a": 1, "b": 2});
    let proof = client
        .prove_circuit(circuit.id(), input, None, None, None)
        .await
        .unwrap();
    assert_eq!(proof.status, JobStatus::Ready);

    client.delete_proof(&proof.proof_id).await.unwrap();

    // Ensure that the proof is no longer available
    let get_result = client.get_proof(&proof.proof_id, None, None, None).await;
    assert!(get_result.is_err());
}
