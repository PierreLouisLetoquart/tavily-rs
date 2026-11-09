use tavily::{MapRequest, Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let tavily = Tavily::from_env()?;

    // 1. List the site's pages, without extracting them (cheaper)
    let request = MapRequest::new("https://docs.tavily.com")
        .select_paths(["/documentation/api-reference/.*"])
        .max_depth(2)
        .limit(30);
    let urls = tavily.call_map(&request).await?.results;
    println!("Found {} API reference pages", urls.len());

    // 2. Extract only those pages, batched by 20
    let pages = tavily.extract_many(urls).await?;
    println!(
        "Extracted {} pages ({} failed)\n",
        pages.results.len(),
        pages.failed_results.len()
    );

    // 3. Ready for an LLM prompt
    println!("{}", pages.to_context(2_000));

    Ok(())
}
