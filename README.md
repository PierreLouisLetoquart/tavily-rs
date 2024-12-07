# Tavily Rust SDK

Unofficial (for now 🫠) Rust SDK for the
[Tavily Search API](https://tavily.com) - the AI-powered search engine for LLM
applications 🚀

> [!NOTE]
> Requires an API key. You can get one by signing up at
> [Tavily](https://app.tavily.com/home). The API key should be kept secure and
> not shared publicly.

## Installation

```bash
cargo add tavily
```

or add it to your `Cargo.toml`:

```toml
[dependencies]
tavily = "^2.0.0"
```

## Quick Start

```rust
use tavily::{Tavily, SearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tavily = Tavily::new("your_api_key");
    
    // Simple search
    let results = tavily.search("Latest AI developments").await?;
    
    // Advanced search with customization
    let mut request = SearchRequest::new("your_api_key", "Breaking tech news");
    request
        .search_depth("advanced")
        .topic("news")
        .include_answer(true)
        .max_results(10);
    
    let results = tavily.call(&request).await?;

    // Extract content from URLs
    let urls = vec![
        "https://tavily.com/",
        "https://github.com/PierreLouisLetoquart",
        "https://www.google.com/",
    ];
    let results = tavily.extract(urls).await?;

    Ok(())
}
```

## Features

- **Simple Search**: Quick search queries with minimal configuration
- **Answer Mode**: Get AI-generated answers along with search results
- **Advanced Customization**: Control search depth, topics, time range, and more
- **Domain Filtering**: Include or exclude specific domains
- **Rich Content**: Optional image results and descriptions

## Documentation

For detailed examples and API documentation, check out:

- [Examples Directory](./examples)
- [API Documentation](https://docs.tavily.com/docs/)
- [Error Codes](https://docs.tavily.com/docs/rest-api/api-reference#error-codes)

## License

[MIT](./LICENSE)
