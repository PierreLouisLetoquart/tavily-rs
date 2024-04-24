mod request;
mod response;

pub use request::SearchRequest;
pub use response::{SearchResponse, SearchResult};

use reqwest::Client;

pub async fn search(request: SearchRequest) -> Result<SearchResponse, reqwest::Error> {
    let client = Client::new();
    let url = "https://api.tavily.com/search";

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;

    Ok(response)
}
