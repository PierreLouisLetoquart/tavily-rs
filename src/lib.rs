//! # Tavily Rust SDK
//!
//! The Tavily Rust SDK simplifies interaction with the Tavily Search API, offering three main functions:
//!
//! Import the library and create a new instance of `Tavily` with your API key.
//!
//! ```rust
//! use tavily::Tavily;
//!
//! let tavily = Tavily::builder("your api key").build()?;
//! ```
//!
//! The `Tavily` instance provides three main functions:
//!
//! - `search`: Quick search with a query string.
//!
//! ```rust
//! let response = tavily.search("your search query").await?;
//! ```
//!
//! - `answer`: Advanced search with query and answer.
//!
//! ```rust
//! let response = tavily.answer("your search query").await?;
//! ```
//!
//! - `call`: Custom search with various options using `SearchRequest`.
//!
//! ```rust
//! use tavily::SearchRequest;
//!
//! let request = SearchRequest::new("your api key", "your search query");
//!     .search_depth("advanced")
//!     .include_answer(true)
//!     .include_images(true)
//!     .include_raw_content(true)
//!     .max_results(10)
//!     .include_domains(vec!["example.com"])
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
