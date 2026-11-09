use tavily::{Result, SearchDepth, SearchRequest, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let tavily = Tavily::from_env()?;

    let question = "What are the main features of the latest Rust release?";
    let request = SearchRequest::new(question)
        .search_depth(SearchDepth::Advanced)
        .max_results(5);
    let response = tavily.call(&request).await?;

    // Ready to send to any LLM
    let prompt = format!(
        "Answer using only the sources below, citing them as [n].\n\n{}\n\nQuestion: {question}",
        response.to_context(8_000)
    );

    println!("{prompt}");

    Ok(())
}
