use reqwest::{Client, ClientBuilder};
use std::time::Duration;

use crate::error::TavilyError;
use crate::request::SearchRequest;
use crate::response::SearchResponse;

const DEFAULT_TIMEOUT: u64 = 30;
const DEFAULT_API_VERSION: &str = "v1";
const BASE_URL: &str = "https://api.tavily.com";

#[derive(Clone)]
pub struct TavilyConfig {
    api_key: String,
    timeout: Duration,
    base_url: String,
    api_version: String,
    max_retries: u32,
}

impl Default for TavilyConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
            base_url: BASE_URL.to_string(),
            api_version: DEFAULT_API_VERSION.to_string(),
            max_retries: 3,
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

    pub fn api_version(mut self, version: &str) -> Self {
        self.config.api_version = version.to_string();
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    pub fn build(self) -> Result<Tavily, TavilyError> {
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
    pub fn builder(api_key: &str) -> TavilyBuilder {
        TavilyBuilder::new(api_key)
    }

    fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{}/{}",
            self.config.base_url, self.config.api_version, path
        )
    }

    async fn call_api(&self, request: &SearchRequest) -> Result<SearchResponse, TavilyError> {
        let url = self.endpoint("search");
        
        let mut retries = 0;
        loop {
            let result = self
                .client
                .post(&url)
                .json(request)
                .send()
                .await?;

            if result.status().is_success() {
                return Ok(result.json::<SearchResponse>().await?);
            }

            // Handle rate limiting
            if result.status() == 429 {
                if retries >= self.config.max_retries {
                    return Err(TavilyError::RateLimit("Rate limit exceeded".into()));
                }
                retries += 1;
                tokio::time::sleep(Duration::from_secs(2u64.pow(retries))).await;
                continue;
            }

            return Err(TavilyError::Api(format!(
                "API request failed: {}",
                result.status()
            )));
        }
    }

    pub async fn search(&self, query: &str) -> Result<SearchResponse, TavilyError> {
        let request = SearchRequest::new(&self.config.api_key, query);
        self.call_api(&request).await
    }

    pub async fn answer(&self, query: &str) -> Result<SearchResponse, TavilyError> {
        let mut request = SearchRequest::new(&self.config.api_key, query);
        request.include_answer(true);
        self.call_api(&request).await
    }
}
