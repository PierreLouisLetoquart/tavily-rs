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
tavily = "^3.0.0"
```

## Quick Start

```rust
use std::time::Duration;

use tavily::{
    ExtractDepth, ExtractRequest, Result, SearchDepth, SearchRequest, SearchTimeRange,
    SearchTopic, Tavily,
};

#[tokio::main]
async fn main() -> Result<()> {
    let tavily = Tavily::builder("tvly-your-api-key")
        .timeout(Duration::from_secs(60))
        .max_retries(5)
        .build()?;

    // Simple search
    let results = tavily.search("Latest AI developments").await?;

    // Advanced search with customization
    let request = SearchRequest::new("Breaking tech news")
        .search_depth(SearchDepth::Advanced)
        .topic(SearchTopic::News)
        .time_range(SearchTimeRange::Week)
        .include_answer(true)
        .max_results(10);

    let results = tavily.call(&request).await?;

    // Extract content from URLs
    let urls = vec![
        "https://tavily.com/",
        "https://github.com/PierreLouisLetoquart",
    ];
    let results = tavily.extract(urls).await?;

    // Extract with customization
    let request = ExtractRequest::new(["https://tavily.com/"])
        .query("pricing")
        .extract_depth(ExtractDepth::Advanced);

    let results = tavily.call_extract(&request).await?;

    Ok(())
}
```

> [!NOTE]
> The API key is sent in the `Authorization: Bearer` header. Requests no longer
> take an API key: since 3.0.0, `SearchRequest::new` and `ExtractRequest::new`
> only take the query or the URLs.

## Features

- **Simple Search**: Quick search queries with minimal configuration
- **Answer Mode**: Get AI-generated answers along with search results
- **Advanced Customization**: Control search depth, topic, time range, country, language and more with typed options
- **Domain Filtering**: Include, prefer or exclude specific domains
- **Rich Content**: Optional raw content, images, descriptions, favicons and published dates
- **Extract Options**: Query-based reranking, extract depth, output format and timeout
- **Credit Usage**: Optional usage information in each response

## Documentation

For detailed examples and API documentation, check out:

- [Examples Directory](./examples)
- [API Documentation](https://docs.tavily.com/)
- [Search Endpoint](https://docs.tavily.com/documentation/api-reference/endpoint/search)
- [Extract Endpoint](https://docs.tavily.com/documentation/api-reference/endpoint/extract)

## License

[MIT](./LICENSE)
