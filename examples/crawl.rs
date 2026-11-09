use tavily::{CrawlRequest, Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let tavily = Tavily::from_env()?;

    let request = CrawlRequest::new("https://docs.tavily.com")
        .instructions("Find all pages about the Python SDK")
        .max_depth(2)
        .limit(10)
        .include_usage(true);

    let response = tavily.call_crawl(&request).await?;

    println!(
        "Crawled {} pages from {}:",
        response.results.len(),
        response.base_url
    );
    for page in &response.results {
        let length = page.raw_content.as_deref().map_or(0, str::len);
        println!("- {} ({length} bytes)", page.url);
    }
    println!(
        "Credits used: {:?}",
        response.usage.map(|usage| usage.credits)
    );

    Ok(())
}
