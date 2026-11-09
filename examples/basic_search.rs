use tavily::{Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let tavily = Tavily::from_env()?;
    let response = tavily.search("What is Rust programming language?").await?;

    println!("Search Results:");
    println!("{:#?}", response);

    Ok(())
}
