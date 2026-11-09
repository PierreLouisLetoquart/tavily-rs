use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// The response from the API when a search is made. [more info](https://docs.tavily.com/documentation/api-reference/endpoint/search)
pub struct SearchResponse {
    pub answer: Option<String>,
    pub query: String,
    pub response_time: f32,
    #[serde(default)]
    pub images: Vec<SearchImageResult>,
    pub results: Vec<SearchResult>,
    /// The parameters selected by Tavily, only present when `auto_parameters` is enabled.
    pub auto_parameters: Option<serde_json::Map<String, serde_json::Value>>,
    pub usage: Option<Usage>,
    pub request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
/// The result type contained in the response. [more info](https://docs.tavily.com/documentation/api-reference/endpoint/search)
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub raw_content: Option<String>,
    pub score: f32,
    pub published_date: Option<String>,
    pub favicon: Option<String>,
    #[serde(default)]
    pub images: Vec<SearchImageResult>,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SearchImageResult {
    Url(String),
    ImageResult {
        url: String,
        description: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
/// The result from the API when an extract is made. [more info](https://docs.tavily.com/documentation/api-reference/endpoint/extract)
pub struct ExtractResult {
    pub results: Vec<ExtractResultItem>,
    #[serde(default)]
    pub failed_results: Vec<FailedExtractResult>,
    pub response_time: f64,
    pub usage: Option<Usage>,
    pub request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExtractResultItem {
    pub url: String,
    pub raw_content: String,
    #[serde(default)]
    pub images: Vec<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FailedExtractResult {
    pub url: String,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub credits: f64,
}
