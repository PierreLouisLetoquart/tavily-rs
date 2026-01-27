use tavily::{Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");

    let tavily = Tavily::builder(&api_key).build()?;
    let response = tavily.search("What is Rust programming language?").await?;

    println!("Search Results:");
    println!("{:#?}", response);

    Ok(())
}
