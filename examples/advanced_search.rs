use tavily::{Tavily, SearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");
    
    let mut request = SearchRequest::new(&api_key, "Latest Rust programming news");
    request.search_depth("advanced");
    request.topic("news");
    request.days(7);
    request.include_answer(true);
    
    let tavily = Tavily::new(&api_key);
    let response = tavily.call(&request).await?;
    
    println!("Advanced Search Results:");
    println!("{:#?}", response);
    
    Ok(())
} 