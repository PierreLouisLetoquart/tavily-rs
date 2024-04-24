use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub answer: Option<String>,
    pub query: String,
    pub response_time: f32,
    pub follow_up_questions: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub raw_content: Option<String>,
    pub score: f32,
}
