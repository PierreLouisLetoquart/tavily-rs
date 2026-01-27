use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// The response from the API when a search is made. [more info](https://docs.tavily.com/docs/tavily-api/rest_api#response)
pub struct SearchResponse {
    pub answer: Option<String>,
    pub query: String,
    pub response_time: f32,
    pub follow_up_questions: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
/// The result type contained in the response. [more info](https://docs.tavily.com/docs/tavily-api/rest_api#response)
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub raw_content: Option<String>,
    pub score: f32,
}

#[derive(Debug, Deserialize)]
/// The result from the API when an extract is made. [more info](https://docs.tavily.com/docs/rest-api/api-reference#request-1)
pub struct ExtractResult {
    pub results: Vec<ExtractResultItem>,
    pub failed_results: Vec<FailedExtractResult>,
    pub response_time: f64,
}

#[derive(Debug, Deserialize)]
pub struct ExtractResultItem {
    pub url: String,
    pub raw_content: String,
}

#[derive(Debug, Deserialize)]
pub struct FailedExtractResult {
    pub url: String,
    pub error: String,
}
