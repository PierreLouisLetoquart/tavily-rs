//! # Tavily Rust SDK
//!
//! The Tavily Rust SDK simplifies interaction with the [Tavily](https://tavily.com) Search and
//! Extract APIs.
//!
//! Import the library and create a new instance of [`Tavily`] with your API key.
//!
//! ```
//! use tavily::Tavily;
//!
//! let tavily = Tavily::builder("tvly-your-api-key").build()?;
//! # Ok::<(), tavily::TavilyError>(())
//! ```
//!
//! or, with a custom timeout and retry policy:
//!
//! ```no_run
//! use std::time::Duration;
//! use tavily::Tavily;
//!
//! let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");
//! let tavily = Tavily::builder(&api_key)
//!     .timeout(Duration::from_secs(60))
//!     .max_retries(5)
//!     .build()?;
//! # Ok::<(), tavily::TavilyError>(())
//! ```
//!
//! The [`Tavily`] instance provides the following functions:
//!
//! - [`search`](Tavily::search): Quick search with a query string.
//!
//! ```no_run
//! # async fn run(tavily: tavily::Tavily) -> tavily::Result<()> {
//! let response = tavily.search("your search query").await?;
//! # Ok(())
//! # }
//! ```
//!
//! - [`answer`](Tavily::answer): Search with an LLM-generated answer.
//!
//! ```no_run
//! # async fn run(tavily: tavily::Tavily) -> tavily::Result<()> {
//! let response = tavily.answer("your search query").await?;
//! println!("{:?}", response.answer);
//! # Ok(())
//! # }
//! ```
//!
//! - [`extract`](Tavily::extract): Extract content from URLs.
//!
//! ```no_run
//! # async fn run(tavily: tavily::Tavily) -> tavily::Result<()> {
//! let response = tavily.extract(["https://example.com", "https://www.rust-lang.org"]).await?;
//! # Ok(())
//! # }
//! ```
//!
//! - [`call`](Tavily::call): Custom search with various options using [`SearchRequest`].
//!
//! ```no_run
//! # async fn run(tavily: tavily::Tavily) -> tavily::Result<()> {
//! use tavily::{SearchDepth, SearchRequest};
//!
//! let request = SearchRequest::new("your search query")
//!     .search_depth(SearchDepth::Advanced)
//!     .include_images(true)
//!     .exclude_domains(["example.org"]);
//!
//! let response = tavily.call(&request).await?;
//! # Ok(())
//! # }
//! ```
//!
//! - [`call_extract`](Tavily::call_extract): Custom extract with various options using
//!   [`ExtractRequest`].
//!
//! ```no_run
//! # async fn run(tavily: tavily::Tavily) -> tavily::Result<()> {
//! use tavily::{ExtractDepth, ExtractRequest};
//!
//! let request = ExtractRequest::new(["https://example.com"])
//!     .query("pricing")
//!     .extract_depth(ExtractDepth::Advanced);
//!
//! let response = tavily.call_extract(&request).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Learn more
//!
//! For examples, error codes and licensing, refer to the [repository](https://github.com/PierreLouisLetoquart/tavily-rs).
mod client;
mod error;
mod request;
mod response;

pub use client::Tavily;
pub use error::{Result, TavilyError};
pub use request::{
    ExtractDepth, ExtractFormat, ExtractRequest, IncludeAnswer, IncludeRawContent, SearchDepth,
    SearchIncludeDomainsMode, SearchRequest, SearchTimeRange, SearchTopic,
};
pub use response::{
    ExtractResult, ExtractResultItem, FailedExtractResult, SearchImageResult, SearchResponse,
    SearchResult, Usage,
};
