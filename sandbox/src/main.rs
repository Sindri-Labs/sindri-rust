use sindri_rs::client::SindriClient;

#[tokio::main]
async fn main() {
    
    let client = SindriClient::new(None);
    let api_keys = client.list_api_keys().await.unwrap();
    // println!("{:?}", api_keys);

    let circuit_info = client.get_circuit("67dc2e04-8a20-46af-aee1-98fbce14f38a").await.unwrap();
    println!("{:?}", circuit_info);

}
