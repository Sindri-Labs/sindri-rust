use std::collections::{HashMap, HashSet};
#[cfg(not(any(feature = "record", feature = "replay")))]
use std::{fs, io::Cursor};

#[cfg(not(any(feature = "record", feature = "replay")))]
use flate2::read::GzDecoder;
#[cfg(not(any(feature = "record", feature = "replay")))]
use tar::Archive;
use tempfile::TempDir;

use sindri::{client::SindriClient, CircuitInfo, CircuitInfoResponse, JobStatus};

mod factory;

#[tokio::test]
async fn test_create_circuit() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None, None);

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

#[test]
fn test_create_circuit_blocking() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::default();

    let test_tags = vec!["tag1".to_string(), "tag2".to_string()];
    let test_meta = HashMap::from([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]);

    let result = client.create_circuit_blocking(
        dir_path.to_string_lossy().to_string(),
        Some(test_tags.clone()),
        Some(test_meta.clone()),
    );

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

    let client = SindriClient::default();
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

#[tokio::test]
async fn test_clone_circuit() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::default();

    let result = client
        .create_circuit(
            dir_path.to_string_lossy().to_string(),
            Some(vec!["clone_test".to_string()]),
            None,
        )
        .await;

    assert!(result.is_ok());
    let circuit = result.unwrap();

    let clone_temp_dir = TempDir::new().unwrap();
    let clone_file_path = clone_temp_dir.path().join("clone.tar.gz");

    let clone_result = client
        .clone_circuit(circuit.id(), clone_file_path.to_string_lossy().to_string())
        .await;
    assert!(clone_result.is_ok());

    // rvcr misinterprets the response body as utf-8 which corrupts the tarball
    // checking the contents of the tarball only enabled for non-vcr feature
    #[cfg(not(any(feature = "record", feature = "replay")))]
    {
        let downloaded = fs::read(clone_file_path).unwrap();
        let cursor = Cursor::new(downloaded);
        let gz_decoder = GzDecoder::new(cursor);
        let mut archive = Archive::new(gz_decoder);

        let file_names: Vec<String> = archive
            .entries()
            .unwrap()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.path().ok().map(|p| p.to_string_lossy().into_owned()))
            .collect();

        assert!(file_names
            .iter()
            .any(|name| name.contains("circuit.circom")));
        assert!(file_names.iter().any(|name| name.contains("sindri.json")));
    }
}
