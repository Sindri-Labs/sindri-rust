use std::time::{Duration, Instant};

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_retry::policies::ExponentialBackoffTimed;
use wiremock::{
    matchers::{header, header_exists, method},
    Mock, MockServer, ResponseTemplate,
};

use crate::{
    client::SindriClient,
    custom_middleware::{retry_client, HeaderDeduplicatorMiddleware},
};

#[tokio::test]
async fn test_client_default_header() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(header_exists("sindri-client"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let outer_client = SindriClient::new(None);
    let inner_client = &outer_client.config().client;

    let request = inner_client.get(mock_server.uri()).build().unwrap();
    let response = inner_client.execute(request).await.unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_header_deduplicator() {
    // Create a request with duplicate headers
    let mut headers = HeaderMap::new();
    headers.append("Authorization", HeaderValue::from_static("Bearer firstkey"));
    headers.append(
        "Authorization",
        HeaderValue::from_static("Bearer secondkey"),
    );
    headers.append("Content-Type", HeaderValue::from_static("application/json"));

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(header("Authorization", "Bearer secondkey"))
        .respond_with(ResponseTemplate::new(400))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(header("Authorization", "Bearer firstkey"))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = reqwest_middleware::ClientBuilder::new(
        reqwest::Client::builder()
            .build()
            .expect("Could not build client"),
    )
    .with(HeaderDeduplicatorMiddleware)
    .build();

    let mut request = client.get(mock_server.uri()).build().unwrap();
    *request.headers_mut() = headers; // manually insert header dupe
    let response = client.execute(request).await.unwrap();

    assert_ne!(response.status(), 404); // If failure, headers do not match positive or negative patterns. Something wrong with the client or wireframe server.
    assert_ne!(response.status(), 400); // If failure, the duplicate header field was not removed. Middleware not working as intended.
    assert_eq!(response.status(), 200); // If failure, one of the headers we wanted to keep isn't there. Check middleware logic, then client.
}

#[tokio::test]
async fn test_retry_policy_on_500() {
    let mock_server = MockServer::start().await;

    // First mock: Return 500 errors for initial N requests
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = reqwest_middleware::ClientBuilder::new(
        reqwest::Client::builder()
            .build()
            .expect("Could not build client"),
    )
    .with(retry_client::<ExponentialBackoffTimed>(Some(
        Duration::from_secs(15),
    )))
    .build();

    // Make the request
    let request = client.get(mock_server.uri()).build().unwrap();
    let start = Instant::now();
    client.execute(request).await.unwrap();
    let elapsed = start.elapsed();

    // Retry logic should make numerous retries in 60 seconds at random deltas
    // between 1s and 8s
    let num_requests = mock_server.received_requests().await.unwrap().len();
    assert!(num_requests > 3);

    // Verify that the duration of retries is about 60 seconds
    let lower_bound = Duration::new(15, 0);
    let upper_bound = Duration::new(25, 0);
    assert!(elapsed >= lower_bound && elapsed <= upper_bound);
}

#[tokio::test]
async fn test_retry_policy_on_400() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(400))
        .mount(&mock_server)
        .await;

    let client = reqwest_middleware::ClientBuilder::new(
        reqwest::Client::builder()
            .build()
            .expect("Could not build client"),
    )
    .with(retry_client::<ExponentialBackoffTimed>(None))
    .build();

    let request = client.get(mock_server.uri()).build().unwrap();
    client.execute(request).await.unwrap();

    let num_retries = mock_server.received_requests().await.unwrap().len();

    assert_eq!(num_retries, 1);
}
