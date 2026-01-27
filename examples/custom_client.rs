use std::time::Duration;

use tavily::{Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");

    let mut builder = reqwest::Client::builder().timeout(Duration::from_secs(60));
    if let Ok(proxy_url) = std::env::var("TAVILY_PROXY") {
        builder = builder.proxy(reqwest::Proxy::all(proxy_url)?);
    }
    let client = builder.build()?;

    let tavily = Tavily::with_reqwest_client(&api_key, client)?;
    let response = tavily.search("Rust async HTTP client").await?;

    println!("Custom Client Search Results:");
    println!("{:#?}", response);

    Ok(())
}
