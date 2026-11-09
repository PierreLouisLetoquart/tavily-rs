use tavily::{Result, Tavily};

#[tokio::main]
async fn main() -> Result<()> {
    let tavily = Tavily::from_env()?;
    let response = tavily
        .extract(vec![
            "https://crates.io/crates/tavily",
            "https://docs.tavily.com/documentation/api-reference/endpoint/extract",
            "https://github.com/PierreLouisLetoquart",
        ])
        .await?;

    println!("Extract Results:");
    println!("{:#?}", response);

    Ok(())
}
