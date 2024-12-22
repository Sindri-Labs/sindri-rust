// use mockito::{Mock, Server};

// fn setup_mock_client(server: &Server) -> SindriClient {
//     let auth_options = AuthOptions {
//         api_key: Some("test_api_key".to_string()),
//         base_url: Some(server.url()),
//     };
//     SindriClient::new(Some(auth_options))
// }



// #[tokio::test]
// async fn test_get_circuit() {
//     let mut server = Server::new();
//     let mock_response = json!({
//         "id": "test_circuit_1",
//         "name": "Test Circuit",
//         "status": "Ready"
//     });

//     let _m = Mock::new("GET", "/api/v1/circuit/test_circuit_1")
//         .with_status(200)
//         .with_header("content-type", "application/json")
//         .with_body(mock_response.to_string())
//         .create_async()
//         .await;

//     let client = setup_mock_client(&server);
//     let result = client.get_circuit("test_circuit_1").await.unwrap();
    
//     assert_eq!(result.id, "test_circuit_1");
// }

// #[tokio::test]
// async fn test_prove_circuit() {
//     let mut server = Server::new();
//     let mock_response = json!({
//         "id": "test_proof_1",
//         "circuit_id": "test_circuit_1",
//         "status": "Pending"
//     });

//     let _m = mock("POST", "/api/v1/circuit/test_circuit_1/prove")
//         .with_status(200)
//         .with_header("content-type", "application/json")
//         .with_body(mock_response.to_string())
//         .create_async()
//         .await;

//     let client = setup_mock_client(&server);
//     let proof_input = r#"{"input": "test_value"}"#;
//     let result = client.prove_circuit("test_circuit_1", proof_input, None, None).await.unwrap();
    
//     assert_eq!(result.id, "test_proof_1");
//     assert_eq!(result.circuit_id, "test_circuit_1");
// } 
