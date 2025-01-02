// use sindri_rs::client::SindriClient;

// #[tokio::test]
// async fn test_create_proof() {
//     let client = SindriClient::new(None);

//     // Run the test
//     let result = client.create_proof("circuit_id", None).await;

//     assert!(result.is_ok());
//     if let Ok(proof) = result {
//         assert!(!proof.proof_id.is_empty());
//     }
// }

// #[tokio::test]
// async fn test_get_proof() {
//     let client = SindriClient::new(None);

//     // First create a proof (this will be recorded)
//     let create_result = client.create_proof("circuit_id", None).await.unwrap();
//     let proof_id = create_result.proof_id;

//     // Then get the proof details
//     let result = client.get_proof(&proof_id).await;

//     assert!(result.is_ok());
//     if let Ok(proof) = result {
//         assert_eq!(proof.proof_id, proof_id);
//     }
// }
