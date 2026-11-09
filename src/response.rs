use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// The response from the API when a search is made. [more info](https://docs.tavily.com/documentation/api-reference/endpoint/search)
pub struct SearchResponse {
    pub answer: Option<String>,
    pub query: String,
    pub response_time: f32,
    #[serde(default)]
    pub images: Vec<SearchImageResult>,
    pub results: Vec<SearchResult>,
    /// The parameters selected by Tavily, only present when `auto_parameters` is enabled.
    pub auto_parameters: Option<serde_json::Map<String, serde_json::Value>>,
    pub usage: Option<Usage>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// The result type contained in the response. [more info](https://docs.tavily.com/documentation/api-reference/endpoint/search)
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub raw_content: Option<String>,
    pub score: f32,
    pub published_date: Option<String>,
    pub favicon: Option<String>,
    #[serde(default)]
    pub images: Vec<SearchImageResult>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum SearchImageResult {
    Url(String),
    #[non_exhaustive]
    ImageResult {
        url: String,
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// The result from the API when an extract is made. [more info](https://docs.tavily.com/documentation/api-reference/endpoint/extract)
pub struct ExtractResult {
    pub results: Vec<ExtractResultItem>,
    #[serde(default)]
    pub failed_results: Vec<FailedExtractResult>,
    pub response_time: f64,
    pub usage: Option<Usage>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ExtractResultItem {
    pub url: String,
    pub title: Option<String>,
    pub raw_content: String,
    #[serde(default)]
    pub images: Vec<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct FailedExtractResult {
    pub url: String,
    pub error: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Usage {
    pub credits: f64,
}

impl SearchResponse {
    /// Format the results as a numbered list of sources, ready to paste into an LLM prompt.
    ///
    /// Each source is rendered as `[n] title`, `Source: url` and its content (the raw content
    /// when it was requested, the snippet otherwise). The output never exceeds `max_chars`
    /// characters: once the budget is reached, the last source is cut with `…` and the
    /// remaining ones are dropped.
    ///
    /// ```no_run
    /// # async fn run(tavily: tavily::Tavily) -> tavily::Result<()> {
    /// let response = tavily.search("Who won the 2026 World Cup?").await?;
    /// let prompt = format!(
    ///     "Answer using only these sources and cite them as [n].\n\n{}",
    ///     response.to_context(8_000)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_context(&self, max_chars: usize) -> String {
        build_context(
            self.results.iter().map(|result| {
                (
                    result.title.as_str(),
                    result.url.as_str(),
                    result.raw_content.as_deref().unwrap_or(&result.content),
                )
            }),
            max_chars,
        )
    }
}

impl ExtractResult {
    /// Merge the results of several batches of the same extract request.
    pub(crate) fn merge(batches: Vec<ExtractResult>, elapsed: std::time::Duration) -> Self {
        let mut merged = ExtractResult {
            results: Vec::new(),
            failed_results: Vec::new(),
            response_time: elapsed.as_secs_f64(),
            usage: None,
            request_id: None,
        };
        for batch in batches {
            merged.results.extend(batch.results);
            merged.failed_results.extend(batch.failed_results);
            if let Some(usage) = batch.usage {
                merged.usage.get_or_insert(Usage { credits: 0.0 }).credits += usage.credits;
            }
        }
        merged
    }

    /// Format the extracted pages as a numbered list of sources, ready to paste into an LLM
    /// prompt. Same format and `max_chars` budget as [`SearchResponse::to_context`]; pages
    /// without a title use their URL as title. Failed URLs are not included.
    pub fn to_context(&self, max_chars: usize) -> String {
        build_context(
            self.results.iter().map(|result| {
                (
                    result.title.as_deref().unwrap_or(&result.url),
                    result.url.as_str(),
                    result.raw_content.as_str(),
                )
            }),
            max_chars,
        )
    }
}

/// Smallest number of content characters worth keeping when the last source has to be cut.
const MIN_TRUNCATED_CONTENT: usize = 20;

fn build_context<'a, I>(sources: I, max_chars: usize) -> String
where
    I: Iterator<Item = (&'a str, &'a str, &'a str)>,
{
    let mut context = String::new();
    let mut used = 0;

    for (index, (title, url, content)) in sources.enumerate() {
        let separator = if context.is_empty() { "" } else { "\n\n" };
        let header = format!(
            "{separator}[{}] {}\nSource: {}\n",
            index + 1,
            title.trim(),
            url
        );
        let content = content.trim();

        let header_len = header.chars().count();
        let content_len = content.chars().count();
        if used + header_len + content_len <= max_chars {
            context.push_str(&header);
            context.push_str(content);
            used += header_len + content_len;
            continue;
        }

        // Keep a cut version of this source only if a meaningful part of its content fits.
        let remaining = max_chars.saturating_sub(used + header_len);
        if remaining > MIN_TRUNCATED_CONTENT {
            context.push_str(&header);
            context.extend(content.chars().take(remaining - 1));
            context.push('…');
        }
        break;
    }

    context
}
