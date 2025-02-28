use std::collections::HashSet;

use sindri::{client::SindriClient, CircuitInfo, CircuitInfoResponse, JobStatus};

#[tokio::test]
async fn end_to_end() {
    // Use prepped tarball since gzip encoding is nondeterministic
    let dir_path = "../cli/tests/factory/circuit.tar.gz";

    let client = SindriClient::new(None, None);

    let test_tags = vec!["tester".to_string()];


    let result = client
        .create_circuit(
            dir_path.to_string(),
            Some(test_tags.clone()),
            None, // Omit metadata since hashmap serialization is nondeterministic
        )
        .await;

    assert!(result.is_ok());
    let circuit = result.unwrap();
    let circuit_identifier = circuit.id();
    
    assert_eq!(*circuit.status(), JobStatus::Ready);
    assert_eq!(
        circuit.tags().iter().collect::<HashSet<_>>(),
        test_tags.iter().collect::<HashSet<_>>()
    );

    // Clone the circuit info before the match to avoid move issues
    let circom_info = match circuit.clone() {
        CircuitInfoResponse::Circom(circom_info) => circom_info,
        _ => panic!("Circuit is not of Circom type"),
    };

    assert_eq!(circom_info.proving_scheme, "groth16");
    assert_eq!(circom_info.num_outputs, Some(1));

    // Proof test
    let input = r#"{"a": 1, "b": 2}"#;
    let result = client
        .prove_circuit(
            circuit_identifier,
            input,
            None,
            None,
            None,
        )
        .await;

    assert!(result.is_ok());
    let proof = result.unwrap();

    assert!(!proof.proof_id.is_empty());

}
