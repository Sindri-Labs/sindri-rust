use std::collections::HashMap;

use sindri_rs::{
    client::SindriClient,
    CircuitInfo,
};

mod factory;

#[tokio::test]
async fn test_create_proof() {

    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None);

    // Due to parallel execution of tests, the circuit with tag "latest"
    // may not exist or be ready yet
    let circuit = client
        .create_circuit(dir_path.to_string_lossy().to_string(), Some(vec!["prove_copy".to_string()]), None)
        .await;

    assert!(circuit.is_ok());
    let circuit = circuit.unwrap();
    let circuit_identifier = circuit.id();

    let input = r#"{"a": 1, "b": 2}"#;
    let test_meta = HashMap::from([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]);
    let result = client.prove_circuit(circuit_identifier, input, Some(test_meta.clone()), None, None).await;

    assert!(result.is_ok());
    let proof = result.unwrap();
    
    assert!(!proof.proof_id.is_empty());
    assert_eq!(proof.meta, test_meta);
}
