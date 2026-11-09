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
tavily = "^4.1.0"
```

## Quick Start

```rust
use std::time::Duration;

use tavily::{
    CrawlRequest, ExtractDepth, ExtractRequest, MapRequest, Result, SearchDepth, SearchRequest,
    SearchTimeRange, SearchTopic, Tavily,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Reads TAVILY_API_KEY (or use `Tavily::builder("tvly-your-api-key")`)
    let tavily = Tavily::builder_from_env()?
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

    // Extract any number of URLs (batched by 20, the API limit)
    let urls: Vec<String> = (1..=50).map(|n| format!("https://example.com/{n}")).collect();
    let results = tavily.extract_many(urls).await?;

    // Crawl a website and extract its pages
    let request = CrawlRequest::new("https://docs.tavily.com")
        .instructions("Find all pages about the Python SDK")
        .max_depth(2)
        .limit(20);
    let results = tavily.call_crawl(&request).await?;

    // List a website's URLs, then extract only the ones you need
    let request = MapRequest::new("https://docs.tavily.com").select_paths(["/documentation/.*"]);
    let urls = tavily.call_map(&request).await?.results;
    let results = tavily.extract_many(urls).await?;

    Ok(())
}
```

Crawl, map and advanced extract can run long on Tavily's side (up to 150
seconds for crawl and map); these requests wait for Tavily's own timeout, even
past the client timeout.

### LLM-ready context

`to_context` turns search, extract or crawl results into numbered sources, capped at a
character budget, ready to drop into a prompt:

```rust
let question = "What changed in the latest Rust release?";
let response = tavily.search(question).await?;

let prompt = format!(
    "Answer using only the sources below, citing them as [n].\n\n{}\n\nQuestion: {question}",
    response.to_context(8_000)
);
```

```text
[1] <title of the first result>
Source: <url of the first result>
<content of the first result>

[2] <title of the second result>
...
```

### Error handling

API errors are typed by cause and carry Tavily's message:

```rust
use tavily::TavilyError;

match tavily.search("rust").await {
    Ok(response) => println!("{} results", response.results.len()),
    Err(TavilyError::Unauthorized(_)) => eprintln!("check your API key"),
    Err(TavilyError::RateLimited { retry_after, .. }) => eprintln!("retry in {retry_after:?}"),
    Err(TavilyError::UsageLimit(err)) => eprintln!("out of credits: {}", err.message),
    Err(err) => eprintln!("{err}"),
}
```

Rate limits (429) and server errors (500, 502, 503, 504) are retried first,
following Tavily's `Retry-After` header when present.

## TLS

HTTPS uses [rustls](https://github.com/rustls/rustls) by default, so no OpenSSL
is needed. To use the platform's native TLS library instead:

```toml
[dependencies]
tavily = { version = "^4.1.0", default-features = false, features = ["native-tls"] }
```

## Upgrading to 4.0

- **Errors**: `TavilyError::RateLimit(String)` and `TavilyError::Api(String)`
  are replaced by `Unauthorized`, `BadRequest`, `RateLimited { error, retry_after }`,
  `UsageLimit` and `Api`, each holding an `ApiError` (`status`, `message`, `body`).
  `err.status()` gives the HTTP status of any API error.
- **reqwest 0.13**: `Tavily::with_reqwest_client` takes a `reqwest` 0.13 `Client`,
  and rustls replaces native TLS as the default backend (see [TLS](#tls)).
- **`#[non_exhaustive]`**: public enums and response structs can gain variants and
  fields in minor releases. `match` statements need a `_` arm, and response
  structs can only be created by deserializing them.

Since 3.0.0, the API key is sent in the `Authorization: Bearer` header, and
`SearchRequest::new` and `ExtractRequest::new` only take the query or the URLs.

## Features

- **Simple Search**: Quick search queries with minimal configuration, API key from `TAVILY_API_KEY`
- **LLM-ready Context**: `to_context` formats results as numbered, budgeted sources for prompts
- **Batch Extract**: `extract_many` handles any number of URLs, in concurrent batches of 20
- **Crawl & Map**: Explore a website with depth, breadth, regex path/domain filters and natural language instructions
- **Resilient**: Retries rate limits and server errors, honoring `Retry-After`
- **Typed Errors**: Unauthorized, bad request, rate limit and usage limit errors with Tavily's message
- **No OpenSSL**: rustls by default, native TLS as an option
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
- [Crawl Endpoint](https://docs.tavily.com/documentation/api-reference/endpoint/crawl)
- [Map Endpoint](https://docs.tavily.com/documentation/api-reference/endpoint/map)

## License

[MIT](./LICENSE)
