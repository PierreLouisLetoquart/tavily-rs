use std::time::Duration;
use tavily::{Tavily, TavilyError};

#[tokio::main]
async fn main() -> Result<(), TavilyError> {
    let client = Tavily::builder("your-api-key")
        .timeout(Duration::from_secs(60))
        .max_retries(5)
        .build()?;

    let response = client.search("What is Rust programming?").await?;
    println!("Search results: {:?}", response);

    Ok(())
} 