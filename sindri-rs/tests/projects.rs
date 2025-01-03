
use sindri_rs::client::SindriClient;

mod factory;

#[tokio::test]
async fn test_create_circuit() {
    let (_temp_dir, dir_path) = factory::baby_circuit();

    let client = SindriClient::new(None);
    let result = client
        .create_circuit(dir_path.to_string_lossy().to_string(), None, None)
        .await;

    assert!(result.is_ok());
}
