use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use futures_util::stream::{self, StreamExt, TryStreamExt};

use reqwest::header::{HeaderMap, RETRY_AFTER};
use reqwest::{Client, ClientBuilder, StatusCode};
use tokio::time::sleep;

use crate::error::{Result, TavilyError};
use crate::request::{ExtractRequest, SearchRequest};
use crate::response::{ExtractResult, SearchResponse};

const DEFAULT_TIMEOUT: u64 = 30;
const DEFAULT_MAX_RETRIES: u32 = 3;
const BASE_URL: &str = "https://api.tavily.com";
const API_KEY_ENV: &str = "TAVILY_API_KEY";
const MAX_URLS_PER_EXTRACT: usize = 20;
const MAX_CONCURRENT_EXTRACTS: usize = 4;
const RETRYABLE_SERVER_ERRORS: [StatusCode; 4] = [
    StatusCode::INTERNAL_SERVER_ERROR,
    StatusCode::BAD_GATEWAY,
    StatusCode::SERVICE_UNAVAILABLE,
    StatusCode::GATEWAY_TIMEOUT,
];

/// Exponential backoff (2, 4, 8… seconds) with up to 250ms of jitter.
fn backoff(retry: u32) -> Duration {
    let jitter_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.subsec_millis() % 250)
        .unwrap_or(0) as u64;
    Duration::from_secs(2u64.saturating_pow(retry)) + Duration::from_millis(jitter_ms)
}

/// The delay requested by the server in the `Retry-After` header, in seconds.
fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    let seconds = headers
        .get(RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()?;
    Some(Duration::from_secs(seconds))
}

#[derive(Clone)]
pub struct TavilyConfig {
    api_key: String,
    timeout: Duration,
    base_url: String,
    max_retries: u32,
}

impl Default for TavilyConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
            base_url: BASE_URL.to_string(),
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }
}

pub struct TavilyBuilder {
    config: TavilyConfig,
}

impl TavilyBuilder {
    pub fn new(api_key: &str) -> Self {
        let config = TavilyConfig {
            api_key: api_key.to_string(),
            ..TavilyConfig::default()
        };
        Self { config }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.config.base_url = url.to_string();
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    pub fn build(self) -> Result<Tavily> {
        if self.config.api_key.is_empty() {
            return Err(TavilyError::Configuration("API key is required".into()));
        }

        let client = ClientBuilder::new()
            .timeout(self.config.timeout)
            .build()
            .map_err(TavilyError::from)?;

        Ok(Tavily {
            config: self.config,
            client,
        })
    }
}

pub struct Tavily {
    config: TavilyConfig,
    client: Client,
}

impl Tavily {
    pub fn builder<S>(api_key: S) -> TavilyBuilder
    where
        S: AsRef<str> + Into<String>,
    {
        TavilyBuilder::new(&api_key.into())
    }

    /// Create a client with default settings, using the API key from the `TAVILY_API_KEY`
    /// environment variable.
    ///
    /// ```no_run
    /// let tavily = tavily::Tavily::from_env()?;
    /// # Ok::<(), tavily::TavilyError>(())
    /// ```
    pub fn from_env() -> Result<Self> {
        Self::builder_from_env()?.build()
    }

    /// Create a builder using the API key from the `TAVILY_API_KEY` environment variable, to
    /// customize the timeout, retries or base URL.
    ///
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// let tavily = tavily::Tavily::builder_from_env()?
    ///     .timeout(Duration::from_secs(60))
    ///     .build()?;
    /// # Ok::<(), tavily::TavilyError>(())
    /// ```
    pub fn builder_from_env() -> Result<TavilyBuilder> {
        let api_key = std::env::var(API_KEY_ENV).map_err(|_| {
            TavilyError::Configuration(format!("{API_KEY_ENV} environment variable is not set"))
        })?;
        Ok(TavilyBuilder::new(&api_key))
    }

    pub fn with_reqwest_client<S>(api_key: S, client: Client) -> Result<Self>
    where
        S: AsRef<str> + Into<String>,
    {
        let config = TavilyConfig {
            api_key: api_key.into(),
            ..TavilyConfig::default()
        };

        if config.api_key.is_empty() {
            return Err(TavilyError::Configuration("API key is required".into()));
        }

        Ok(Self { config, client })
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.config.base_url, path)
    }

    async fn call_api<T, R>(&self, endpoint: &str, request: &T) -> Result<R>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let url = self.endpoint(endpoint);

        let mut retries = 0;
        loop {
            let sent = self
                .client
                .post(&url)
                .bearer_auth(&self.config.api_key)
                .json(request)
                .send()
                .await;

            // Connection failures never reached Tavily, so they are safe to retry. Timeouts are
            // not retried: the request may have been processed and its credits charged.
            let response = match sent {
                Ok(response) => response,
                Err(err) if err.is_connect() && retries < self.config.max_retries => {
                    retries += 1;
                    sleep(backoff(retries)).await;
                    continue;
                }
                Err(err) => return Err(err.into()),
            };

            let status = response.status();
            if status.is_success() {
                return Ok(response.json::<R>().await?);
            }

            let retryable = status == StatusCode::TOO_MANY_REQUESTS
                || RETRYABLE_SERVER_ERRORS.contains(&status);
            if retryable && retries < self.config.max_retries {
                retries += 1;
                let delay = retry_after(response.headers()).unwrap_or_else(|| backoff(retries));
                sleep(delay).await;
                continue;
            }

            let body = response
                .text()
                .await
                .unwrap_or_else(|err| format!("<failed to read body: {}>", err));
            if status == StatusCode::TOO_MANY_REQUESTS {
                return Err(TavilyError::RateLimit(format!(
                    "Rate limit exceeded: {} - {}",
                    status, body
                )));
            }
            return Err(TavilyError::Api(format!(
                "API request failed: {} - {}",
                status, body
            )));
        }
    }

    pub async fn call(&self, body: &SearchRequest) -> Result<SearchResponse> {
        self.call_api("search", body).await
    }

    /// Extract content from URLs with custom options using an `ExtractRequest`.
    pub async fn call_extract(&self, body: &ExtractRequest) -> Result<ExtractResult> {
        self.call_api("extract", body).await
    }

    /// Extract content from any number of URLs, working around the API limit of 20 URLs per
    /// request. See [`Tavily::call_extract_many`].
    pub async fn extract_many<I, S>(&self, urls: I) -> Result<ExtractResult>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.call_extract_many(&ExtractRequest::new(urls)).await
    }

    /// Like [`Tavily::call_extract`], but for any number of URLs.
    ///
    /// The URLs are split into batches of 20 (the API limit) that share the request's options,
    /// with up to 4 batches in flight at once. The results are merged into one
    /// [`ExtractResult`]: `results` and `failed_results` keep the batch order, `usage` adds up
    /// the credits, `response_time` is the total wall-clock time, and `request_id` is `None`
    /// since each batch has its own. If any batch fails (e.g. invalid API key), the error is
    /// returned. Requests with 20 URLs or fewer are sent as is.
    ///
    /// ```no_run
    /// # async fn run(tavily: tavily::Tavily) -> tavily::Result<()> {
    /// let urls: Vec<String> = (1..=100).map(|n| format!("https://example.com/{n}")).collect();
    /// let response = tavily.extract_many(urls).await?;
    /// println!("{} pages, {} failures", response.results.len(), response.failed_results.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_extract_many(&self, body: &ExtractRequest) -> Result<ExtractResult> {
        if body.urls().len() <= MAX_URLS_PER_EXTRACT {
            return self.call_extract(body).await;
        }

        let started = Instant::now();
        let batches: Vec<ExtractResult> = stream::iter(body.urls().chunks(MAX_URLS_PER_EXTRACT))
            .map(|urls| {
                let request = body.with_urls(urls.to_vec());
                async move { self.call_extract(&request).await }
            })
            .buffered(MAX_CONCURRENT_EXTRACTS)
            .try_collect()
            .await?;

        Ok(ExtractResult::merge(batches, started.elapsed()))
    }

    pub async fn search<S>(&self, query: S) -> Result<SearchResponse>
    where
        S: AsRef<str> + Into<String>,
    {
        let request = SearchRequest::new(query.into());
        self.call_api("search", &request).await
    }

    pub async fn answer<S>(&self, query: S) -> Result<SearchResponse>
    where
        S: AsRef<str> + Into<String>,
    {
        let request = SearchRequest::new(query.into()).include_answer(true);
        self.call_api("search", &request).await
    }

    pub async fn extract<I, S>(&self, urls: I) -> Result<ExtractResult>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        let request = ExtractRequest::new(urls);
        self.call_api("extract", &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_after_reads_seconds() {
        let mut headers = HeaderMap::new();
        assert_eq!(retry_after(&headers), None);

        headers.insert(RETRY_AFTER, "60".parse().unwrap());
        assert_eq!(retry_after(&headers), Some(Duration::from_secs(60)));

        headers.insert(
            RETRY_AFTER,
            "Wed, 21 Oct 2026 07:28:00 GMT".parse().unwrap(),
        );
        assert_eq!(retry_after(&headers), None);
    }

    #[test]
    fn backoff_grows_exponentially() {
        assert!(backoff(1) >= Duration::from_secs(2) && backoff(1) < Duration::from_secs(3));
        assert!(backoff(3) >= Duration::from_secs(8) && backoff(3) < Duration::from_secs(9));
    }
}
