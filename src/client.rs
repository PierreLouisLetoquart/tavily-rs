use reqwest::Client;

pub struct Tavily {
    api_key: String,
    client: Client,
}

impl Tavily {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    async fn call_api(&self, request: &SearchRequest) -> Result<SearchResponse, reqwest::Error> {
        let url = "https://api.tavily.com/search";

        let response = self
            .client
            .post(url)
            .json(request)
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

        Ok(response)
    }

    pub async fn call(&self, body: &SearchRequest) -> Result<SearchResponse, reqwest::Error> {
        let response = self.call_api(&body).await?;
        Ok(response)
    }

    pub async fn search(&self, query: &str) -> Result<SearchResponse, reqwest::Error> {
        let request = SearchRequest::new(&self.api_key, query);
        let response = Self::call_api(&self, &request).await?;
        Ok(response)
    }

    pub async fn answer(&self, query: &str) -> Result<SearchResponse, reqwest::Error> {
        let mut request = SearchRequest::new(&self.api_key, query);
        request.include_answer(true);
        let response = Self::call_api(&self, &request).await?;
        Ok(response)
    }
}
