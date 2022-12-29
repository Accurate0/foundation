use anyhow::Context;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_tracing::TracingMiddleware;
use std::time::Duration;

fn get_default_middleware_http_client(
    client: reqwest::Client,
) -> reqwest_middleware::ClientWithMiddleware {
    get_default_middleware(client).build()
}

pub fn get_default_middleware(client: reqwest::Client) -> reqwest_middleware::ClientBuilder {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(2);
    reqwest_middleware::ClientBuilder::new(client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .with(TracingMiddleware::default())
}

pub fn get_default_http_client() -> reqwest_middleware::ClientWithMiddleware {
    get_http_client(get_default_middleware_http_client)
}

pub fn get_http_client<T>(wrap_in_middleware: T) -> reqwest_middleware::ClientWithMiddleware
where
    T: Fn(reqwest::Client) -> reqwest_middleware::ClientWithMiddleware,
{
    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()
        .context("Failed to build http client")
        .unwrap();
    wrap_in_middleware(client)
}

pub fn get_http_client_with_headers(
    headers: http::HeaderMap,
) -> reqwest_middleware::ClientWithMiddleware {
    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()
        .context("Failed to build http client")
        .unwrap();
    get_default_middleware_http_client(client)
}
