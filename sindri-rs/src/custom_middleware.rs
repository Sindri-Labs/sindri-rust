//! Custom middleware definitions for the SindriClient.
//!
//! Important types of middleware implemented here:
//!
//! - `HeaderDeduplicatorMiddleware`: Removes duplicate headers from requests (bugfix for openapi client).
//! - `LoggingMiddleware`: Logs requests and responses.
//! - `Retry500`: Implements a retry policy for 500-series errors.
//! - `VCRMiddleware`: Records and replays requests for (internal) testing purposes.

use std::{collections::HashSet, time::Duration, Arc};

use async_compression::tokio::write::ZstdEncoder;
use async_trait::async_trait;
use http::Extensions;
use reqwest::{
    header::HeaderValue, header::CONTENT_ENCODING, Body, Request, Request, Response, Response,
    StatusCode,
};
use reqwest_middleware::{Middleware, Next, Result};
use reqwest_retry::{
    default_on_request_failure,
    policies::{ExponentialBackoff, ExponentialBackoffTimed},
    RetryTransientMiddleware, Retryable, RetryableStrategy,
};
#[cfg(any(feature = "record", feature = "replay"))]
use rvcr::{VCRMiddleware, VCRMode};
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;
use tracing::{debug, error};

const ZSTD_BUFFER_SIZE: usize = 4096;
const ZSTD_MIN_BODY_SIZE: usize = 512;

pub struct HeaderDeduplicatorMiddleware;

/// Custom middleware to deduplicate headers
///
/// The openapi client adds the bearer token twice, so we need to deduplicate it.
#[async_trait::async_trait]
impl Middleware for HeaderDeduplicatorMiddleware {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<reqwest::Response, reqwest_middleware::Error> {
        // Get headers as mutable map
        let headers = req.headers_mut();
        let mut seen = HashSet::new();
        let mut to_remove = Vec::new();

        // Collect duplicate headers
        for (name, _) in headers.iter() {
            if !seen.insert(name) {
                to_remove.push(name.clone());
            }
        }

        // Remove duplicates, only keeping first occurrence
        for name in to_remove {
            let values: Vec<_> = headers.get_all(&name).iter().cloned().collect();
            headers.remove(&name);
            if let Some(first_value) = values.first() {
                headers.insert(&name, first_value.clone());
            }
        }

        next.run(req, extensions).await
    }
}

pub struct LoggingMiddleware;

/// Simple logging of requests and responses
/// Always attached to a Sindri Client but only invoked when `RUST_LOG=debug`
/// and a tracing subscriber is attached to the global logger.
#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        debug!("Request sent: {:?}", req);
        let res = next.run(req, extensions).await;
        debug!("Response received: {:?}", res);
        res
    }
}

pub struct Retry500;
impl RetryableStrategy for Retry500 {
    fn handle(
        &self,
        res: &Result<reqwest::Response, reqwest_middleware::Error>,
    ) -> Option<Retryable> {
        // Middleware retry classification rules

        // 400s are not retried because they are client errors, and
        // there is no reason to believe that the same request will
        // succeed on a retry. Also true for some 5xx errors.
        let unrecoverable_codes = [
            StatusCode::from_u16(400).unwrap(),
            StatusCode::from_u16(401).unwrap(),
            StatusCode::from_u16(403).unwrap(),
            StatusCode::from_u16(404).unwrap(),
            StatusCode::from_u16(405).unwrap(),
            StatusCode::from_u16(406).unwrap(),
            StatusCode::from_u16(407).unwrap(),
            StatusCode::from_u16(408).unwrap(),
            StatusCode::from_u16(409).unwrap(),
            StatusCode::from_u16(410).unwrap(),
            StatusCode::from_u16(411).unwrap(),
            StatusCode::from_u16(412).unwrap(),
            StatusCode::from_u16(413).unwrap(),
            StatusCode::from_u16(414).unwrap(),
            StatusCode::from_u16(415).unwrap(),
            StatusCode::from_u16(416).unwrap(),
            StatusCode::from_u16(417).unwrap(),
            StatusCode::from_u16(418).unwrap(),
            StatusCode::from_u16(421).unwrap(),
            StatusCode::from_u16(501).unwrap(),
            StatusCode::from_u16(505).unwrap(),
            StatusCode::from_u16(506).unwrap(),
            StatusCode::from_u16(510).unwrap(),
        ];

        // 500, 502, 503, 504 indicate a server error that may soon be resolved.
        let transient_codes = [
            StatusCode::from_u16(500).unwrap(),
            StatusCode::from_u16(502).unwrap(),
            StatusCode::from_u16(503).unwrap(),
            StatusCode::from_u16(504).unwrap(),
        ];

        match res {
            // retry if temporary API outage: 500, 502, 503, or 504
            Ok(success) if transient_codes.contains(&success.status()) => {
                debug!(
                    "Retrying request due to temporary API outage: {}",
                    success.status()
                );
                Some(Retryable::Transient)
            }
            // cause a panic if client error: 400s
            Ok(success) if unrecoverable_codes.contains(&success.status()) => {
                debug!(
                    "Request failed with fatal client error: {}",
                    success.status()
                );
                Some(Retryable::Fatal)
            }
            // otherwise do not retry a successful request (even for 400s/300s)
            Ok(_success) => None,
            // but maybe retry a request failure due to local network issue
            Err(error) => {
                debug!("Request failed with network error: {}", error);
                default_on_request_failure(error)
            }
        }
    }
}

/// Returns a HTTP client which will retry requests with response errors meeting the retry500 "transient error" classification
/// Default behavior is a retry at random times between 1s and 8s for a default maximum duration of 60s.
///
/// The retry policy is configurable with `max_duration` which defaults to 60s.
pub fn retry_client(
    max_duration: Option<Duration>,
) -> RetryTransientMiddleware<ExponentialBackoffTimed, Retry500> {
    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(8))
        .build_with_total_retry_duration(max_duration.unwrap_or(Duration::from_secs(60)));
    RetryTransientMiddleware::new_with_policy_and_strategy(retry_policy, Retry500)
}

/// Returns record & replay middleware for testing purposes
#[cfg(any(feature = "record", feature = "replay"))]
pub fn vcr_middleware(bundle: std::path::PathBuf) -> VCRMiddleware {
    let mut vcr = VCRMiddleware::try_from(bundle.clone()).unwrap();

    vcr = vcr.with_modify_request(|req| {
        // Redact Bearer token in Authorization header before saving
        req.headers.insert(
            "authorization".to_string(),
            vec!["Bearer REDACTED_TOKEN".to_string()],
        );
    });

    vcr = vcr.with_modify_response(|res| {
        if res
            .headers
            .get("content-type")
            .and_then(|values| values.first())
            .map(|v| v.contains("application/octet-stream"))
            .unwrap_or(false)
        {
            res.body.encoding = None; // Do not attempt to base64 decode any octet-stream data
        }
    });

    #[cfg(feature = "record")]
    {
        vcr = vcr.with_mode(VCRMode::Record);
    }

    vcr
}

#[derive(Debug)]
pub struct ZstdRequestCompressionMiddleware;

#[async_trait]
impl Middleware for ZstdRequestCompressionMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        // If the request has a sizable body, compress it using zstd.
        if let Some(bytes) = req
            .body()
            .and_then(|b| b.as_bytes())
            .filter(|b| b.len() >= ZSTD_MIN_BODY_SIZE)
        {
            // Create a new request with the same properties.
            let (method, url, headers, version) = (
                req.method().clone(),
                req.url().clone(),
                req.headers().clone(),
                req.version(),
            );
            let mut new_req = Request::new(method, url);
            *new_req.headers_mut() = headers;
            *new_req.version_mut() = version;

            // Swap out the body with a zstd compressed stream of the original.
            let (writer, reader) = tokio::io::duplex(ZSTD_BUFFER_SIZE);
            let body_arc = Arc::new(bytes.to_vec());
            let body_clone = Arc::clone(&body_arc);
            tokio::spawn(async move {
                let mut encoder = ZstdEncoder::new(writer);
                if let Err(error) = encoder.write_all(&body_clone).await {
                    error!("Failed to compress body: {}", error);
                }
                let _ = encoder.shutdown().await;
            });
            new_req
                .body_mut()
                .replace(Body::wrap_stream(ReaderStream::new(reader)));

            // Set the `Content-Encoding: zstd` header.
            new_req
                .headers_mut()
                .insert(CONTENT_ENCODING, HeaderValue::from_static("zstd"));

            // Proceed with the new request.
            return next.run(new_req, extensions).await;
        }

        // If no body needs to be compressed, proceed with the original request.
        next.run(req, extensions).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{Duration, Instant};

    use reqwest::header::{HeaderMap, HeaderValue};
    use wiremock::{
        matchers::{header, method},
        Mock, MockServer, ResponseTemplate,
    };

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
        .with(retry_client(Some(Duration::from_secs(15))))
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
        .with(retry_client(None))
        .build();

        let request = client.get(mock_server.uri()).build().unwrap();
        client.execute(request).await.unwrap();

        let num_retries = mock_server.received_requests().await.unwrap().len();

        assert_eq!(num_retries, 1);
    }

    #[tokio::test]
    async fn test_zstd_request_compression() {
        let mock_server = MockServer::start().await;

        let original_body = "A".repeat(ZSTD_MIN_BODY_SIZE);

        let mock = Mock::given(method("POST"))
            .and(header("Content-Encoding", "zstd"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount_as_scoped(&mock_server)
            .await;

        let client = reqwest_middleware::ClientBuilder::new(
            reqwest::Client::builder()
                .build()
                .expect("Could not build client"),
        )
        .with(ZstdRequestCompressionMiddleware)
        .build();

        let request = client
            .post(mock_server.uri())
            .body(original_body.to_string())
            .build()
            .unwrap();
        let response = client.execute(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let received_request = mock_server.received_requests().await.unwrap();
        assert_eq!(received_request.len(), 1);
        let compressed_body = &received_request[0].body;
        let decompressed_body = zstd::decode_all(&compressed_body[..]).unwrap();
        assert_eq!(
            decompressed_body,
            original_body.as_bytes(),
            "Decompressed body does not match original input."
        );
    }
}
