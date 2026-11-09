use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
/// The depth of the search ("ultra-fast", "fast", "basic" or "advanced")
pub enum SearchDepth {
    UltraFast,
    Fast,
    Basic,
    Advanced,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
/// The category of the search ("general", "news" or "finance")
pub enum SearchTopic {
    General,
    News,
    Finance,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
/// The time range back from the current date to filter results based on publish date or last updated date
pub enum SearchTimeRange {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
/// Controls how include_domains is applied
pub enum SearchIncludeDomainsMode {
    Restrict,
    Prefer,
}

#[derive(Default, Debug, Serialize)]
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
/// The depth of the extract.
pub enum ExtractDepth {
    Basic,
    Advanced,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
/// The format of the extracted web page content.
pub enum ExtractFormat {
    Markdown,
    Text,
}

#[derive(Default, Debug, Serialize)]
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

    /// Chunks are short content snippets (about 800 characters each) pulled directly from the source. Use chunks_per_source to define the maximum number of relevant chunks returned per source and to control the raw_content length. Available only when query is provided. Default is 3.
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

    /// The depth of the search ("ultra-fast", "fast", "basic" or "advanced"). Default is basic for quick results and advanced for indepth high quality results but longer response time.
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

    /// The number of maximum search results to return. Default is 5.
    pub fn max_results(mut self, max_results: i32) -> Self {
        self.max_results = Some(max_results);
        self
    }

    /// A list of domains to specifically include in the search results. Default is None, which includes all domains.
    pub fn include_domains<I, S>(mut self, include_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.include_domains = Some(include_domains.into_iter().map(Into::into).collect());
        self
    }

    /// A list of domains to specifically exclude from the search results. Default is None, which doesn't exclude any domains.
    pub fn exclude_domains<I, S>(mut self, exclude_domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + Into<String>,
    {
        self.exclude_domains = Some(exclude_domains.into_iter().map(Into::into).collect());
        self
    }

    /// Set the category of the search ("general" or "news"). Default is "general".
    pub fn topic(mut self, topic: SearchTopic) -> Self {
        self.topic = Some(topic);
        self
    }

    /// When include_images is set to True, this option adds descriptive text for each image. Default is False.
    pub fn include_image_descriptions(mut self, include_descriptions: bool) -> Self {
        self.include_image_descriptions = Some(include_descriptions);
        self
    }

    /// Chunks are short content snippets (about 500 characters each for basic and fast, 800 for advanced) pulled directly from the source. Use chunks_per_source to define the maximum number of relevant chunks returned per source and to control the content length. Available when search_depth is advanced, basic or fast. Default is 3.
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

    /// The maximum age in hours of Tavily's cached copy of each result.
    pub fn max_age_hours(mut self, max_age_hours: i32) -> Self {
        self.max_age_hours = Some(max_age_hours);
        self
    }

    /// Whether to include the favicon URL for each result. Default is False.
    pub fn include_favicon(mut self, include_favicon: bool) -> Self {
        self.include_favicon = Some(include_favicon);
        self
    }

    /// Controls how include_domains is applied. restrict limits results to only the listed domains. prefer also searches the rest of the web, so results outside include_domains can still surface, rather than excluding them. Default is restrict.
    pub fn include_domains_mode(mut self, include_domains_mode: SearchIncludeDomainsMode) -> Self {
        self.include_domains_mode = Some(include_domains_mode);
        self
    }

    /// Boost search results from a specific country. This will prioritize content from the selected country in the search results. Available only if topic is general.
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

    /// Strictly filter out search results that don't match the language parameter, instead of only boosting them in ranking. Default is False.
    pub fn filter_by_language(mut self, filter_by_language: bool) -> Self {
        self.filter_by_language = Some(filter_by_language);
        self
    }

    /// When auto_parameters is enabled, Tavily automatically configures search parameters based on your query's content and intent. You can still set other parameters manually, and your explicit values will override the automatic ones. Default is False.
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
