# Tavily Rust Library

This library provides an unofficial Rust binding for the Tavily Search API. It allows you to easily perform search queries and handle responses.

Here's a simple example of how to use the library:

```rust
use tavily::{search, SearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = SearchRequest {
        api_key: "your_api_key".to_string(),
        query: "your_search_query".to_string(),
        search_depth: Some("basic".to_string()),
        include_answer: Some(false),
        include_images: Some(true),
        include_raw_content: Some(false),
        max_results: Some(5),
        include_domains: Some(vec![]),
        exclude_domains: Some(vec![]),
    };

    let response = search(request).await?;
    println!("{:#?}", response);

    Ok(())
}
```

## Error Codes

The Tavily Search API may return various HTTP status codes. For a complete list and their meanings, please refer to the [official doc](https://docs.tavily.com/docs/tavily-api/rest_api#error-codes).

## Disclaimer

This is an unofficial binding for the Tavily Search API. For the official documentation and support, please visit [Tavily Search API](https://tavily.com).

## License

MIT

