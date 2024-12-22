use sindri_rs::client::SindriClient;

#[tokio::main]
async fn main() {
    
    let client = SindriClient::new(None);

    let circuit_info = client.get_circuit("67dc2e04-8a20-46af-aee1-98fbce14f38a").await.unwrap();
    println!("{:?}", circuit_info);

    let input = r#"{"X": "1", "Y": "1"}"#;
    let proof_info = client.prove_circuit("67dc2e04-8a20-46af-aee1-98fbce14f38a", input, None, None).await.unwrap();
    println!("{:?}", proof_info);
}
