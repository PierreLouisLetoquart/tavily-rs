use tavily::Tavily;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");
    
    let tavily = Tavily::new(&api_key);
    let response = tavily.search("What is Rust programming language?").await?;
    
    println!("Search Results:");
    println!("{:#?}", response);
    
    Ok(())
} 