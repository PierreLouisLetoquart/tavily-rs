use std::time::Duration;

use tavily::{Result, SearchDepth, SearchRequest, SearchTimeRange, SearchTopic, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Tavily::builder_from_env()?
        .timeout(Duration::from_secs(60))
        .max_retries(5)
        .build()?;

    let request = SearchRequest::new("Latest Rust programming news")
        .search_depth(SearchDepth::Advanced)
        .topic(SearchTopic::News)
        .time_range(SearchTimeRange::Week)
        .include_answer(true);

    let response = client.call(&request).await?;

    println!("Advanced Search Results:");
    println!("{:#?}", response);

    Ok(())
}
