use tavily::{Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");

    let tavily = Tavily::builder(&api_key).build()?;
    let response = tavily
        .extract(vec![
            "https://crates.io/crates/tavily",
            "https://docs.tavily.com/docs/rest-api/api-reference",
            "https://github.com/PierreLouisLetoquart",
        ])
        .await?;

    println!("Extract Results:");
    println!("{:#?}", response);

    Ok(())
}
