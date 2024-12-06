# Tavily Rust SDK

The Tavily Rust SDK is a library for interacting with the Tavily Search API.
With just a few lines of code, you can perform simple or advanced search
queries, customize your search options, and get relevant search results powered
by LLMs 🚀.

> **Note:** Required an [api key](https://app.tavily.com/home)

## Functions

The Tavily Rust SDK provides three main functions:

- `search`: Perform a simple search query with a single argument, `query`.

```rust
let response = tavily.search("your search query").await?;
```

- `answer`: Perform an advanced search query that includes an answer to your
  query. This function takes a bit more time than the simple search.

```rust
let response = tavily.answer("your search query").await?;
```

- `call`: Perform a custom search query using the `SearchRequest` struct. This
  struct provides a range of options for customizing your search:

```rust
let mut request = SearchRequest::new("your api key", "your search query");
request.search_depth("advanced");          // "basic" or "advanced"
request.topic("news");                     // "general" or "news"
request.days(7);                           // Only for "news" topic
request.include_answer(true);
request.include_images(true);
request.include_image_descriptions(true);
request.include_raw_content(true);
request.max_results(10);
request.include_domains(vec!["example.com".to_string()]);
request.exclude_domains(vec!["example.org".to_string()]);

let response = tavily.call(&request).await?;
```

## Getting Started

To get started with the Tavily Rust SDK, add the following to your `Cargo.toml`
file:

```rust
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
    request.topic("news");
    request.days(7);
    request.include_answer(true);
    request.include_images(true);
    request.include_image_descriptions(true);
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

The Tavily Search API may return various HTTP status codes. For a complete list
and their meanings, please refer to the
[official documentation](https://docs.tavily.com/docs/tavily-api/rest_api#error-codes).

## Disclaimer

This is an unofficial SDK for the Tavily Search API. For the official
documentation and support, please visit [Tavily Search API](https://tavily.com).

## License

[MIT](./LICENSE)

## TODO (for august 2024)

- check new beta RAG feature
  [here](https://docs.tavily.com/docs/python-sdk/tavily-hybrid-rag/api-reference)
- check the actual og py ref for improvment and update from tavily
  [here](https://docs.tavily.com/docs/python-sdk/tavily-search/api-reference)

- [ ] Detailed todo list with subtasks.

- [ ] Add more examples and use cases to the README.
- [ ] Write tests.
