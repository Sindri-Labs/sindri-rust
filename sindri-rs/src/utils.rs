use reqwest_middleware::{Middleware, Next};
use task_local_extensions::Extensions;
use std::collections::HashSet;

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
