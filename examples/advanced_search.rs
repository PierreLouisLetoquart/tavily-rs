use std::time::Duration;

use tavily::{Result, SearchRequest, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");

    let client = Tavily::builder(&api_key)
        .timeout(Duration::from_secs(60))
        .max_retries(5)
        .build()?;

    let request = SearchRequest::new(&api_key, "Latest Rust programming news")
        .search_depth("advanced")
        .topic("news")
        .days(7)
        .include_answer(true);

    let response = client.call(&request).await?;

    println!("Advanced Search Results:");
    println!("{:#?}", response);

    Ok(())
}
