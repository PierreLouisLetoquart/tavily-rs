use reqwest::{Client, ClientBuilder};
use std::time::Duration;

use crate::error::{Result, TavilyError};
use crate::request::{ExtractRequest, SearchRequest};
use crate::response::{ExtractResult, SearchResponse};

const DEFAULT_TIMEOUT: u64 = 30;
const DEFAULT_MAX_RETRIES: u32 = 3;
const BASE_URL: &str = "https://api.tavily.com";

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
        let mut config = TavilyConfig::default();
        config.api_key = api_key.to_string();
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
            let result = self.client.post(&url).json(request).send().await?;

            if result.status().is_success() {
                return Ok(result.json::<R>().await?);
            }

            // Handle rate limiting
            if result.status() == 429 {
                if retries >= self.config.max_retries {
                    return Err(TavilyError::RateLimit("Rate limit exceeded".into()));
                }
                retries += 1;
                std::thread::sleep(std::time::Duration::from_secs(2u64.pow(retries)));
                continue;
            }

            return Err(TavilyError::Api(format!(
                "API request failed: {}",
                result.status()
            )));
        }
    }

    pub async fn call(&self, body: &SearchRequest) -> Result<SearchResponse> {
        self.call_api("search", body).await
    }

    pub async fn search<S>(&self, query: S) -> Result<SearchResponse>
    where
        S: AsRef<str> + Into<String>,
    {
        let request = SearchRequest::new(&self.config.api_key, &query.into());
        self.call_api("search", &request).await
    }

    pub async fn answer<S>(&self, query: S) -> Result<SearchResponse>
    where
        S: AsRef<str> + Into<String>,
    {
        let request = SearchRequest::new(&self.config.api_key, &query.into()).include_answer(true);
        self.call_api("search", &request).await
    }

    pub async fn extract<I, S>(&self, urls: I) -> Result<ExtractResult>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        let request = ExtractRequest::new(&self.config.api_key, urls);
        self.call_api("extract", &request).await
    }
}
