# Tavily Rust SDK

The Tavily Rust SDK is a powerful and easy-to-use library for interacting with the Tavily Search API 🚀

## Getting Started

To get started with the Tavily Rust SDK, add the following to your `Cargo.toml` file:

```txt
[dependencies]
tavily = "^1.0.0"
```

Then, import the library in your Rust code:

```rust
use tavily::{Tavily, SearchRequest, SearchResponse};
```

Here's a simple example of how to use the library to perform a search query:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "your_api_key";
    let query = "your_search_query";

    let tavily = Tavily::new(api_key);
    let response = tavily.search(query).await?;

    println!("{:#?}", response);

    Ok(())
}
```

You can also customize the search options by using the `SearchRequest` struct:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "your_api_key";
    let query = "your_search_query";

    let mut request = SearchRequest::new(api_key, query);
    request.search_depth("advanced");
    request.include_answer(true);
    request.include_images(true);
    request.include_raw_content(true);
    request.max_results(10);
    request.include_domains(vec!["example.com".to_string()]);
    request.exclude_domains(vec!["example.org".to_string()]);

    let tavily = Tavily::new(api_key);
    let response = tavily.call(&request).await?;

    println!("{:#?}", response);

    Ok(())
}
```

## Error Codes

The Tavily Search API may return various HTTP status codes. For a complete list and their meanings, please refer to the [official documentation](https://docs.tavily.com/docs/tavily-api/rest_api#error-codes).

## Disclaimer

This is an unofficial SDK for the Tavily Search API. For the official documentation and support, please visit [Tavily Search API](https://tavily.com).

## License

MIT
