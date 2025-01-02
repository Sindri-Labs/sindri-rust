use sindri_rs::client::SindriClient;

#[tokio::test]
async fn test_create_circuit() {
    let client = SindriClient::new(None);

    let result = client.create_circuit("circuit_id".to_string(), None, None).await;
    
    assert!(result.is_ok());
}