use std::{collections::HashSet, time::Duration};

use http::Extensions;
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Middleware, Next};
use reqwest_retry::{
    default_on_request_failure,
    policies::{ExponentialBackoff, ExponentialBackoffTimed},
    RetryTransientMiddleware, Retryable, RetryableStrategy,
};

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

/// Simple debug logging of reqwest headers and bodies plus raw response
#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        tracing::debug!("[LOGGING MIDDLEWARE] Request started {:?}", req);
        let res = next.run(req, extensions).await;
        tracing::debug!("[LOGGING MIDDLEWARE] Result: {:?}", res);
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
                tracing::debug!(
                    "[RETRY MIDDLEWARE] Retrying request due to temporary API outage: {}",
                    success.status()
                );
                Some(Retryable::Transient)
            }
            // cause a panic if client error: 400s
            Ok(success) if unrecoverable_codes.contains(&success.status()) => {
                tracing::debug!(
                    "[RETRY MIDDLEWARE] Request failed with fatal client error: {}",
                    success.status()
                );
                Some(Retryable::Fatal)
            }
            // otherwise do not retry a successful request (even for 400s/300s)
            Ok(_success) => None,
            // but maybe retry a request failure due to local network issue
            Err(error) => {
                tracing::debug!(
                    "[RETRY MIDDLEWARE] Request failed with network error: {}",
                    error
                );
                default_on_request_failure(error)
            }
        }
    }
}

// Returns a robust HTTP client with ExponentialBackoff on retries up to max_duration
pub fn retry_client<T: reqwest_retry::RetryPolicy + std::marker::Sync + std::marker::Send>(
    max_duration: Option<Duration>,
) -> RetryTransientMiddleware<ExponentialBackoffTimed, Retry500> {
    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(8))
        .build_with_total_retry_duration(max_duration.unwrap_or(Duration::from_secs(60)));
    RetryTransientMiddleware::new_with_policy_and_strategy(retry_policy, Retry500)
}
