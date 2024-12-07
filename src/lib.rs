//! # Tavily Rust SDK
//!
//! The Tavily Rust SDK simplifies interaction with the Tavily Search API, offering three main functions:
//!
//! Import the library and create a new instance of `Tavily` with your API key.
//!
//! ```rust,ignore
//! use tavily::Tavily;
//!
//! let tavily = Tavily::builder("tvly-your-api-key").build()?;
//! ```
//!
//! or
//!
//! ```rust,ignore
//! let api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");
//! let tavily = Tavily::builder(&api_key)
//!     .timeout(Duration::from_secs(60))
//!     .max_retries(5)
//!     .build()?;
//! ```
//!
//! The `Tavily` instance provides the following functions:
//!
//! - `search`: Quick search with a query string.
//!
//! ```rust,ignore
//! let response = tavily.search("your search query").await?;
//! ```
//!
//! - `answer`: Advanced search with query and answer.
//!
//! ```rust,ignore
//! let response = tavily.answer("your search query").await?;
//! ```
//!
//! - `extract`: Extract content from a URL.
//!
//! ```rust,ignore
//! let response = tavily.extract(vec!["https://example.com", "..."]).await?;
//! ```
//!
//! - `call`: Custom search with various options using `SearchRequest`.
//!
//! ```rust,ignore
//! use tavily::SearchRequest;
//!
//! let request = SearchRequest::new(&api_key, "your search query")
//!     .search_depth("advanced")
//!     .include_images(true)
//!     .exclude_domains(vec!["example.org"]);
//!
//! let response = tavily.call(&request).await?;
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
pub use request::SearchRequest;
pub use response::{SearchResponse, SearchResult};
