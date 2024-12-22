use openapi::apis::authorization_api::apikey_list;
use sindri_rs::client::SindriClient;
use tracing_subscriber;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

#[tokio::main]
async fn main() {
    env_logger::init();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    let client = SindriClient::new(None);
    let api_keys = client.list_api_keys().await.unwrap();
    println!("{:?}", api_keys);

}
