use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub api_key: String,
    pub query: String,
    pub search_depth: Option<String>,
    pub include_answer: Option<bool>,
    pub include_images: Option<bool>,
    pub include_raw_content: Option<bool>,
    pub max_results: Option<i32>,
    pub include_domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
}

