use crate::custom_middleware::HeaderDeduplicatorMiddleware;
use reqwest::header::{HeaderMap, HeaderValue};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, header};



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


// #[tokio::test]
// async fn test_new_client_default_headers() {
//     let client = SindriClient::new(None);
//     let config = client.config();

    // Example code from openapi/src/apis/authorization_api.rs
    // This code is used in nearly all methods. However it isn't modularized by the codegen
    // So we can't call a method from that package.
    // let local_var_configuration = client.config();

    // let local_var_client = &local_var_configuration.client;

    // // TODO: change so it's not hitting api
    // let local_var_uri_str = format!("{}/api/v1/sindri-manifest-schema.json", local_var_configuration.base_path);
    // let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    // if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
    //     local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    // }
    // if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
    //     local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    // };
    // if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
    //     local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    // };

    // let local_var_req = local_var_req_builder.build().unwrap();
    // println!("{:?}", local_var_req.headers());
    // let local_var_resp = local_var_client.execute(local_var_req).await.unwrap();
    // let headers = local_var_resp.
    // .request().headers();
    // println!("{:?}", headers);

    // println!("{:?}",headers);

//     assert_eq!(headers.get("User-Agent").unwrap(), "OpenAPI-Generator/v1.14.5/rust");
//     assert!(headers.get("Authorization").unwrap().to_str().unwrap().starts_with("Bearer "));
//     assert!(headers.contains_key("sindri-client"));

// }
