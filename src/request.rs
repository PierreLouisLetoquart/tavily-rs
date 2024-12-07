use serde::Serialize;

#[derive(Debug, Serialize)]
/// Request object for the search API [more info](https://docs.tavily.com/docs/tavily-api/rest_api#parameters)
pub struct SearchRequest {
    api_key: String,
    query: String,
    search_depth: Option<String>,
    include_answer: Option<bool>,
    include_images: Option<bool>,
    include_raw_content: Option<bool>,
    max_results: Option<i32>,
    include_domains: Option<Vec<String>>,
    exclude_domains: Option<Vec<String>>,
    topic: Option<String>,
    days: Option<i32>,
    include_image_descriptions: Option<bool>,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            api_key: "".into(),
            query: "".into(),
            search_depth: Some("basic".into()),
            include_answer: Some(false),
            include_images: Some(false),
            include_raw_content: Some(false),
            max_results: Some(5),
            include_domains: Some(vec![]),
            exclude_domains: Some(vec![]),
            topic: Some("general".into()),
            days: Some(3),
            include_image_descriptions: Some(false),
        }
    }
}

impl SearchRequest {
    /// Create a new search request
    pub fn new(api_key: &str, query: &str) -> Self {
        Self {
            api_key: api_key.into(),
            query: query.into(),
            ..Default::default()
        }
    }

    /// The depth of the search ("basic" or "advanced"). Default is basic for quick results and advanced for indepth high quality results but longer response time.
    pub fn search_depth(mut self, search_depth: &str) -> Self {
        self.search_depth = Some(search_depth.into());
        self
    }

    /// Include answers in the search results. Default is False.
    pub fn include_answer(mut self, include_answer: bool) -> Self {
        self.include_answer = Some(include_answer);
        self
    }

    /// Include a list of query related images in the response. Default is False.
    pub fn include_images(mut self, include_images: bool) -> Self {
        self.include_images = Some(include_images);
        self
    }

    /// Include raw content in the search results. Default is False.
    pub fn include_raw_content(mut self, include_raw_content: bool) -> Self {
        self.include_raw_content = Some(include_raw_content);
        self
    }

    /// The number of maximum search results to return. Default is 5.
    pub fn max_results(mut self, max_results: i32) -> Self {
        self.max_results = Some(max_results);
        self
    }

    /// A list of domains to specifically include in the search results. Default is None, which includes all domains.
    pub fn include_domains(mut self, include_domains: Vec<String>) -> Self {
        self.include_domains = Some(include_domains);
        self
    }

    /// A list of domains to specifically exclude from the search results. Default is None, which doesn't exclude any domains.
    pub fn exclude_domains(mut self, exclude_domains: Vec<String>) -> Self {
        self.exclude_domains = Some(exclude_domains);
        self
    }

    /// Set the category of the search ("general" or "news"). Default is "general".
    pub fn topic(mut self, topic: &str) -> Self {
        self.topic = Some(topic.into());
        self
    }

    /// Set the number of days back from the current date to include in search results.
    /// Only available when using the "news" search topic. Default is 3.
    pub fn days(mut self, days: i32) -> Self {
        self.days = Some(days);
        self
    }

    /// When include_images is set to True, this option adds descriptive text for each image. Default is False.
    pub fn include_image_descriptions(mut self, include_descriptions: bool) -> Self {
        self.include_image_descriptions = Some(include_descriptions);
        self
    }
}
