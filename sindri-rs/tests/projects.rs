use std::collections::{HashMap, HashSet};

use sindri_rs::{
    CircuitInfo, CircuitInfoResponse, client::SindriClient, JobStatus
};

mod factory;

#[tokio::test]
async fn test_create_circuit() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None);

    let test_tags = vec!["tag1".to_string(), "tag2".to_string()];
    let test_meta = HashMap::from([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]);

    let result = client
        .create_circuit(
            dir_path.to_string_lossy().to_string(),
            Some(test_tags.clone()),
            Some(test_meta.clone()),
        )
        .await;

    assert!(result.is_ok());
    let circuit = result.unwrap();

    assert_eq!(*circuit.status(), JobStatus::Ready);
    assert_eq!(circuit.meta(), &test_meta);
    assert_eq!(
        circuit.tags().iter().collect::<HashSet<_>>(),
        test_tags.iter().collect::<HashSet<_>>()
    );

    let circom_info = match circuit {
        CircuitInfoResponse::Circom(circom_info) => circom_info,
        _ => panic!("Circuit is not of Circom type"),
    };

    assert_eq!(circom_info.proving_scheme, "groth16");
    assert_eq!(circom_info.num_outputs, Some(1));
}


#[tokio::test]
async fn test_delete_circuit() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None);
    let result = client
        .create_circuit(
            dir_path.to_string_lossy().to_string(),
            Some(vec!["circuit_deletion".to_string()]),
            None,
        )
        .await;

    assert!(result.is_ok());
    let circuit = result.unwrap();

    let delete_result = client.delete_circuit(circuit.id()).await;
    assert!(delete_result.is_ok());

    // Ensure that the circuit is no longer available
    let get_result = client.get_circuit(circuit.id(), None).await;
    assert!(get_result.is_err());
}
