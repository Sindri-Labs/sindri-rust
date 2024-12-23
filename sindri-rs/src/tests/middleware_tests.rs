use crate::custom_middleware::HeaderDeduplicatorMiddleware;
use reqwest::header::{HeaderMap, HeaderValue};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{header, header_exists, method};
use crate::client::SindriClient;


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
    headers.append("Authorization", HeaderValue::from_static("Bearer secondkey"));
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
            .expect("Could not build client")
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



// #[tokio::test]
// async fn test_retry500_strategy() {
//     let mock_server = MockServer::start().await;
    
//     // Test transient error (500)
//     Mock::given(any())
//         .respond_with(ResponseTemplate::new(500))
//         .mount(&mock_server)
//         .await;
    
//     let retry_strategy = Retry500;
//     let client = reqwest::Client::new();
//     let response = client.get(mock_server.uri()).send().await.unwrap();
    
//     let result = retry_strategy.handle(&Ok(response));
//     assert_eq!(result, Some(Retryable::Transient));
    
//     // Test unrecoverable error (400)
//     Mock::given(any())
//         .respond_with(ResponseTemplate::new(400))
//         .mount(&mock_server)
//         .await;
    
//     let response = client.get(mock_server.uri()).send().await.unwrap();
//     let result = retry_strategy.handle(&Ok(response));
//     assert_eq!(result, Some(Retryable::Fatal));
    
//     // Test successful response (200)
//     Mock::given(any())
//         .respond_with(ResponseTemplate::new(200))
//         .mount(&mock_server)
//         .await;
    
//     let response = client.get(mock_server.uri()).send().await.unwrap();
//     let result = retry_strategy.handle(&Ok(response));
//     assert_eq!(result, None);
// }

// #[tokio::test]
// async fn test_retry_client() {
//     let max_duration = Duration::from_secs(30);
//     let retry_middleware = retry_client(Some(max_duration));
    
//     // Verify retry policy configuration
//     let policy = retry_middleware.get_policy();
//     assert_eq!(policy.get_total_retry_duration(), Some(max_duration));
// }

// #[tokio::test]
// async fn test_logging_middleware() {
//     let mock_server = MockServer::start().await;
    
//     Mock::given(any())
//         .respond_with(ResponseTemplate::new(200))
//         .mount(&mock_server)
//         .await;
    
//     let client = reqwest::Client::new();
//     let middleware = LoggingMiddleware;
//     let mut extensions = Extensions::new();
//     let req = client.get(mock_server.uri()).build().unwrap();
    
//     // Test that logging doesn't affect the response
//     let next = Next::new(&|req, _| Box::pin(async move {
//         Ok(Response::new(req))
//     }));
    
//     let result = middleware.handle(req, &mut extensions, next).await;
//     assert!(result.is_ok());
// }
