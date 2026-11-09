use std::time::Duration;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Whether to include an LLM-generated answer (`true`/`false`, "basic" or "advanced")
pub enum IncludeAnswer {
    Bool(bool),
    Basic,
    Advanced,
}

impl From<bool> for IncludeAnswer {
    fn from(value: bool) -> Self {
        IncludeAnswer::Bool(value)
    }
}

impl Serialize for IncludeAnswer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            IncludeAnswer::Bool(value) => serializer.serialize_bool(*value),
            IncludeAnswer::Basic => serializer.serialize_str("basic"),
            IncludeAnswer::Advanced => serializer.serialize_str("advanced"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Whether to include the raw content of each result (`true`/`false`, "markdown" or "text")
pub enum IncludeRawContent {
    Bool(bool),
    Markdown,
    Text,
}

impl From<bool> for IncludeRawContent {
    fn from(value: bool) -> Self {
        IncludeRawContent::Bool(value)
    }
}

impl Serialize for IncludeRawContent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            IncludeRawContent::Bool(value) => serializer.serialize_bool(*value),
            IncludeRawContent::Markdown => serializer.serialize_str("markdown"),
            IncludeRawContent::Text => serializer.serialize_str("text"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
/// The depth of the search ("ultra-fast", "fast", "basic" or "advanced")
pub enum SearchDepth {
    UltraFast,
    Fast,
    Basic,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
/// The category of the search ("general", "news" or "finance")
pub enum SearchTopic {
    General,
    News,
    Finance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
/// The time range back from the current date to filter results based on publish date or last updated date
pub enum SearchTimeRange {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
/// Controls how include_domains is applied
pub enum SearchIncludeDomainsMode {
    Restrict,
    Prefer,
}

#[derive(Default, Debug, Clone, Serialize)]
/// Request object for the search API [more info](https://docs.tavily.com/documentation/api-reference/endpoint/search)
pub struct SearchRequest {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_depth: Option<SearchDepth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_answer: Option<IncludeAnswer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_raw_content: Option<IncludeRawContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<SearchTopic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_image_descriptions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunks_per_source: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_range: Option<SearchTimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_published_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_by_published_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_hours: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_favicon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_domains_mode: Option<SearchIncludeDomainsMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_by_language: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_parameters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    safe_search: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
/// The depth of the extract.
pub enum ExtractDepth {
    Basic,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
/// The format of the extracted web page content.
pub enum ExtractFormat {
    Markdown,
    Text,
}

#[derive(Default, Debug, Clone, Serialize)]
/// Request object for the extract API [more info](https://docs.tavily.com/documentation/api-reference/endpoint/extract)
pub struct ExtractRequest {
    urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunks_per_source: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extract_depth: Option<ExtractDepth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_favicon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ExtractFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
}

impl ExtractRequest {
    /// How long Tavily may work on this request before timing out on its side.
    pub(crate) fn server_timeout(&self) -> Duration {
        match (self.timeout, self.extract_depth) {
            (Some(timeout), _) => Duration::from_secs_f32(timeout.clamp(1.0, 60.0)),
            (None, Some(ExtractDepth::Advanced)) => Duration::from_secs(30),
            (None, _) => Duration::from_secs(10),
        }
    }

    pub(crate) fn urls(&self) -> &[String] {
        &self.urls
    }

    /// A copy of this request with the same options, for another set of URLs.
    pub(crate) fn with_urls(&self, urls: Vec<String>) -> Self {
        Self {
            urls,
            ..self.clone()
        }
    }

    /// Create a new extract request for 1 to 20 URLs
    pub fn new<I, S>(urls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            urls: urls.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// User intent for reranking extracted content chunks. When provided, chunks are reranked based on relevance to this query.
    pub fn query<S>(mut self, query: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        self.query = Some(query.into());
        self
    }

    /// Chunks are short content snippets (about 800 characters each) pulled directly from the source. Use chunks_per_source to define the maximum number of relevant chunks returned per source and to control the raw_content length. Available only when query is provided. Must be between 1 and 5. Default is 3.
    pub fn chunks_per_source(mut self, chunks_per_source: i32) -> Self {
        self.chunks_per_source = Some(chunks_per_source);
        self
    }

    /// The depth of the extraction process. advanced extraction retrieves more data, including tables and embedded content, with higher success but may increase latency. Default is basic.
    pub fn extract_depth(mut self, extract_depth: ExtractDepth) -> Self {
        self.extract_depth = Some(extract_depth);
        self
    }

    /// Include a list of images extracted from the URLs in the response. Default is False.
    pub fn include_images(mut self, include_images: bool) -> Self {
        self.include_images = Some(include_images);
        self
    }

    /// Whether to include the favicon URL for each result. Default is False.
    pub fn include_favicon(mut self, include_favicon: bool) -> Self {
        self.include_favicon = Some(include_favicon);
        self
    }

    /// The format of the extracted web page content. markdown returns content in markdown format. text returns plain text and may increase latency. Default is markdown.
    pub fn format(mut self, format: ExtractFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Maximum time in seconds to wait for the URL extraction before timing out. Must be between 1.0 and 60.0 seconds. If not specified, default timeouts are applied based on extract_depth.
    pub fn timeout(mut self, timeout: f32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Whether to include credit usage information in the response. Default is False.
    pub fn include_usage(mut self, include_usage: bool) -> Self {
        self.include_usage = Some(include_usage);
        self
    }
}

impl SearchRequest {
    /// Create a new search request
    pub fn new<S>(query: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }

    /// The depth of the search ("ultra-fast", "fast", "basic" or "advanced"). Default is basic. advanced costs 2 API credits, the others 1.
    pub fn search_depth(mut self, search_depth: SearchDepth) -> Self {
        self.search_depth = Some(search_depth);
        self
    }

    /// Include an LLM-generated answer to the query. `true` or `IncludeAnswer::Basic` returns a quick answer, `IncludeAnswer::Advanced` a more detailed one. Default is False.
    pub fn include_answer<A>(mut self, include_answer: A) -> Self
    where
        A: Into<IncludeAnswer>,
    {
        self.include_answer = Some(include_answer.into());
        self
    }

    /// Include a list of query related images in the response. Default is False.
    pub fn include_images(mut self, include_images: bool) -> Self {
        self.include_images = Some(include_images);
        self
    }

    /// Include the cleaned and parsed HTML content of each result. `true` or `IncludeRawContent::Markdown` returns markdown, `IncludeRawContent::Text` plain text (may increase latency). Default is False.
    pub fn include_raw_content<R>(mut self, include_raw_content: R) -> Self
    where
        R: Into<IncludeRawContent>,
    {
        self.include_raw_content = Some(include_raw_content.into());
        self
    }

    /// The maximum number of search results to return, between 0 and 20. Default is 10.
    pub fn max_results(mut self, max_results: i32) -> Self {
        self.max_results = Some(max_results);
        self
    }

    /// A list of domains to specifically include in the search results (maximum 300). Default is None, which includes all domains.
    pub fn include_domains<I, S>(mut self, include_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.include_domains = Some(include_domains.into_iter().map(Into::into).collect());
        self
    }

    /// A list of domains to specifically exclude from the search results (maximum 150). Default is None, which doesn't exclude any domains.
    pub fn exclude_domains<I, S>(mut self, exclude_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.exclude_domains = Some(exclude_domains.into_iter().map(Into::into).collect());
        self
    }

    /// Set the category of the search ("general", "news" or "finance"). "finance" is not supported for fast or ultra-fast search depths (the API returns a 400 error). Default is "general".
    pub fn topic(mut self, topic: SearchTopic) -> Self {
        self.topic = Some(topic);
        self
    }

    /// When include_images is set to True, this option adds descriptive text for each image. Default is False.
    pub fn include_image_descriptions(mut self, include_descriptions: bool) -> Self {
        self.include_image_descriptions = Some(include_descriptions);
        self
    }

    /// Chunks are short content snippets (about 500 characters each for basic and fast, 800 for advanced) pulled directly from the source. Use chunks_per_source to define the maximum number of relevant chunks returned per source and to control the content length. Available when search_depth is advanced, basic or fast. Must be between 1 and 3. Default is 3.
    pub fn chunks_per_source(mut self, chunks_per_source: i32) -> Self {
        self.chunks_per_source = Some(chunks_per_source);
        self
    }

    /// The time range back from the current date to filter results based on publish date or last updated date. By default, results with no detectable published date are not removed; set filter_by_published_date to true to remove them.
    pub fn time_range(mut self, time_range: SearchTimeRange) -> Self {
        self.time_range = Some(time_range);
        self
    }

    /// Will return all results after the specified start date based on publish date or last updated date. Required to be written in the format YYYY-MM-DD. By default, results with no detectable published date are not removed; set filter_by_published_date to true to remove them.
    pub fn start_date<S>(mut self, start_date: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        self.start_date = Some(start_date.into());
        self
    }

    /// Will return all results before the specified end date based on publish date or last updated date. Required to be written in the format YYYY-MM-DD. By default, results with no detectable published date are not removed; set filter_by_published_date to true to remove them.
    pub fn end_date<S>(mut self, end_date: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        self.end_date = Some(end_date.into());
        self
    }

    /// Include a published_date field in each result. Automatically enabled when topic is news. Default is False.
    pub fn include_published_date(mut self, include_published_date: bool) -> Self {
        self.include_published_date = Some(include_published_date);
        self
    }

    /// Remove results whose published date falls outside the time_range, start_date, or end_date window. Results with no detectable published date are also removed. Setting this to true also enables include_published_date. Default is False.
    pub fn filter_by_published_date(mut self, filter_by_published_date: bool) -> Self {
        self.filter_by_published_date = Some(filter_by_published_date);
        self
    }

    /// The maximum age in hours of Tavily's cached copy of each result. 1 to 720 only returns a cached copy fetched within that many hours, 0 always fetches the page live, -1 never fetches live. Unset (default) serves cached copies and fetches live as needed.
    pub fn max_age_hours(mut self, max_age_hours: i32) -> Self {
        self.max_age_hours = Some(max_age_hours);
        self
    }

    /// Whether to include the favicon URL for each result. Default is False.
    pub fn include_favicon(mut self, include_favicon: bool) -> Self {
        self.include_favicon = Some(include_favicon);
        self
    }

    /// Controls how include_domains is applied. restrict limits results to only the listed domains. prefer also searches the rest of the web, so results outside include_domains can still surface, rather than excluding them. Setting it without include_domains returns a 400 error. Default is restrict.
    pub fn include_domains_mode(mut self, include_domains_mode: SearchIncludeDomainsMode) -> Self {
        self.include_domains_mode = Some(include_domains_mode);
        self
    }

    /// Boost search results from a specific country. This will prioritize content from the selected country in the search results. Expects a lowercase English country name (e.g. "france", "united states"). Available only if topic is general, and not supported for fast or ultra-fast search depths (the API returns a 400 error).
    pub fn country<S>(mut self, country: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        self.country = Some(country.into());
        self
    }

    /// Boost search results in a specific language. Accepts an ISO 639-1 code (e.g. en, fr, zh-cn) or an English language name (e.g. english, french). By default this only boosts matching-language results in ranking; pass filter_by_language: true to strictly filter out non-matching results instead.
    pub fn language<S>(mut self, language: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        self.language = Some(language.into());
        self
    }

    /// Strictly filter out search results that don't match the language parameter, instead of only boosting them in ranking. Requires language to be set; returns a 400 error otherwise. Default is False.
    pub fn filter_by_language(mut self, filter_by_language: bool) -> Self {
        self.filter_by_language = Some(filter_by_language);
        self
    }

    /// When auto_parameters is enabled, Tavily automatically configures search parameters based on your query's content and intent. You can still set other parameters manually, and your explicit values will override the automatic ones. search_depth may be set to advanced (2 API credits); set it explicitly to basic to avoid that. Default is False.
    pub fn auto_parameters(mut self, auto_parameters: bool) -> Self {
        self.auto_parameters = Some(auto_parameters);
        self
    }

    /// Ensure that only search results containing the exact quoted phrase(s) in the query are returned, bypassing synonyms or semantic variations. Default is False.
    pub fn exact_match(mut self, exact_match: bool) -> Self {
        self.exact_match = Some(exact_match);
        self
    }

    /// Whether to include credit usage information in the response. Default is False.
    pub fn include_usage(mut self, include_usage: bool) -> Self {
        self.include_usage = Some(include_usage);
        self
    }

    /// Whether to filter out adult or unsafe content from results. Not supported for fast or ultra-fast search depths. Default is False.
    pub fn safe_search(mut self, safe_search: bool) -> Self {
        self.safe_search = Some(safe_search);
        self
    }
}

#[derive(Default, Debug, Clone, Serialize)]
/// Request object for the crawl API [more info](https://docs.tavily.com/documentation/api-reference/endpoint/crawl)
pub struct CrawlRequest {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunks_per_source: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_depth: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_breadth: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    select_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    select_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extract_depth: Option<ExtractDepth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ExtractFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_favicon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
}

/// Tavily's server-side timeout for crawl and map requests, in seconds: 10 to 150, 150 by default.
fn crawl_server_timeout(timeout: Option<f32>) -> Duration {
    Duration::from_secs_f32(timeout.unwrap_or(150.0).clamp(10.0, 150.0))
}

fn collect_strings<I, S>(items: I) -> Option<Vec<String>>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    Some(items.into_iter().map(Into::into).collect())
}

impl CrawlRequest {
    /// Create a new crawl request starting from the root URL
    pub fn new<S>(url: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    pub(crate) fn server_timeout(&self) -> Duration {
        crawl_server_timeout(self.timeout)
    }

    /// Natural language instructions for the crawler (e.g. "Find all pages about the Python SDK"). When specified, the cost increases to 2 API credits per 10 successful pages instead of 1.
    pub fn instructions<S>(mut self, instructions: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        self.instructions = Some(instructions.into());
        self
    }

    /// Maximum number of relevant chunks (about 800 characters each) returned per page, which controls the raw_content length. Available only when instructions are provided. Must be between 1 and 5. Default is 3.
    pub fn chunks_per_source(mut self, chunks_per_source: i32) -> Self {
        self.chunks_per_source = Some(chunks_per_source);
        self
    }

    /// Max depth of the crawl: how far from the base URL the crawler can explore. Must be between 1 and 5. Default is 1.
    pub fn max_depth(mut self, max_depth: i32) -> Self {
        self.max_depth = Some(max_depth);
        self
    }

    /// Max number of links to follow per level of the tree (i.e. per page). Must be between 1 and 500. Default is 20.
    pub fn max_breadth(mut self, max_breadth: i32) -> Self {
        self.max_breadth = Some(max_breadth);
        self
    }

    /// Total number of links the crawler will process before stopping. Default is 50.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Regex patterns to select only URLs with specific path patterns (e.g. `/docs/.*`, `/api/v1.*`).
    pub fn select_paths<I, S>(mut self, select_paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.select_paths = collect_strings(select_paths);
        self
    }

    /// Regex patterns to restrict crawling to specific domains or subdomains (e.g. `^docs\.example\.com$`).
    pub fn select_domains<I, S>(mut self, select_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.select_domains = collect_strings(select_domains);
        self
    }

    /// Regex patterns to exclude URLs with specific path patterns (e.g. `/private/.*`, `/admin/.*`).
    pub fn exclude_paths<I, S>(mut self, exclude_paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.exclude_paths = collect_strings(exclude_paths);
        self
    }

    /// Regex patterns to exclude specific domains or subdomains from crawling (e.g. `^private\.example\.com$`).
    pub fn exclude_domains<I, S>(mut self, exclude_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.exclude_domains = collect_strings(exclude_domains);
        self
    }

    /// Whether to include external domain links in the final results list. Default is true.
    pub fn allow_external(mut self, allow_external: bool) -> Self {
        self.allow_external = Some(allow_external);
        self
    }

    /// Whether to include images in the crawl results. Default is false.
    pub fn include_images(mut self, include_images: bool) -> Self {
        self.include_images = Some(include_images);
        self
    }

    /// The depth of the extraction. advanced retrieves more data, including tables and embedded content, with higher success but may increase latency. basic costs 1 credit per 5 successful extractions, advanced 2. Default is basic.
    pub fn extract_depth(mut self, extract_depth: ExtractDepth) -> Self {
        self.extract_depth = Some(extract_depth);
        self
    }

    /// The format of the extracted page content. text returns plain text and may increase latency. Default is markdown.
    pub fn format(mut self, format: ExtractFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Whether to include the favicon URL for each result. Default is false.
    pub fn include_favicon(mut self, include_favicon: bool) -> Self {
        self.include_favicon = Some(include_favicon);
        self
    }

    /// Maximum time in seconds to wait for the crawl before timing out. Must be between 10 and 150 seconds. Default is 150.
    pub fn timeout(mut self, timeout: f32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Whether to include credit usage information in the response. The value may be 0 until the minimum usage is reached. Default is false.
    pub fn include_usage(mut self, include_usage: bool) -> Self {
        self.include_usage = Some(include_usage);
        self
    }
}

#[derive(Default, Debug, Clone, Serialize)]
/// Request object for the map API [more info](https://docs.tavily.com/documentation/api-reference/endpoint/map)
pub struct MapRequest {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_depth: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_breadth: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    select_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    select_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_usage: Option<bool>,
}

impl MapRequest {
    /// Create a new map request starting from the root URL
    pub fn new<S>(url: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    pub(crate) fn server_timeout(&self) -> Duration {
        crawl_server_timeout(self.timeout)
    }

    /// Natural language instructions for the crawler (e.g. "Find all pages about the Python SDK"). When specified, the cost increases to 2 API credits per 10 successful pages instead of 1.
    pub fn instructions<S>(mut self, instructions: S) -> Self
    where
        S: AsRef<str> + Into<String>,
    {
        self.instructions = Some(instructions.into());
        self
    }

    /// Max depth of the mapping: how far from the base URL the crawler can explore. Must be between 1 and 5. Default is 1.
    pub fn max_depth(mut self, max_depth: i32) -> Self {
        self.max_depth = Some(max_depth);
        self
    }

    /// Max number of links to follow per level of the tree (i.e. per page). Must be between 1 and 500. Default is 20.
    pub fn max_breadth(mut self, max_breadth: i32) -> Self {
        self.max_breadth = Some(max_breadth);
        self
    }

    /// Total number of links the crawler will process before stopping. Default is 50.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Regex patterns to select only URLs with specific path patterns (e.g. `/docs/.*`, `/api/v1.*`).
    pub fn select_paths<I, S>(mut self, select_paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.select_paths = collect_strings(select_paths);
        self
    }

    /// Regex patterns to restrict mapping to specific domains or subdomains (e.g. `^docs\.example\.com$`).
    pub fn select_domains<I, S>(mut self, select_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.select_domains = collect_strings(select_domains);
        self
    }

    /// Regex patterns to exclude URLs with specific path patterns (e.g. `/private/.*`, `/admin/.*`).
    pub fn exclude_paths<I, S>(mut self, exclude_paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.exclude_paths = collect_strings(exclude_paths);
        self
    }

    /// Regex patterns to exclude specific domains or subdomains from mapping (e.g. `^private\.example\.com$`).
    pub fn exclude_domains<I, S>(mut self, exclude_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.exclude_domains = collect_strings(exclude_domains);
        self
    }

    /// Whether to include external domain links in the final results list. Default is true.
    pub fn allow_external(mut self, allow_external: bool) -> Self {
        self.allow_external = Some(allow_external);
        self
    }

    /// Maximum time in seconds to wait for the mapping before timing out. Must be between 10 and 150 seconds. Default is 150.
    pub fn timeout(mut self, timeout: f32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Whether to include credit usage information in the response. The value may be 0 until 10 pages have been mapped. Default is false.
    pub fn include_usage(mut self, include_usage: bool) -> Self {
        self.include_usage = Some(include_usage);
        self
    }
}
