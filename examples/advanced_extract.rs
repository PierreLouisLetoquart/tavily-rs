use tavily::{ExtractDepth, ExtractFormat, ExtractRequest, Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let tavily = Tavily::from_env()?;

    let request = ExtractRequest::new(["https://www.rust-lang.org/learn"])
        .query("async programming")
        .chunks_per_source(3)
        .extract_depth(ExtractDepth::Advanced)
        .format(ExtractFormat::Text)
        .include_usage(true);

    let response = tavily.call_extract(&request).await?;

    println!("Advanced Extract Results:");
    println!("{:#?}", response);

    Ok(())
}
