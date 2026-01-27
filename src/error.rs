use std::fmt;

pub type Result<T> = core::result::Result<T, TavilyError>;

#[derive(Debug)]
pub enum TavilyError {
    Api(String),
    Http(reqwest::Error),
    Configuration(String),
    RateLimit(String),
}

impl std::error::Error for TavilyError {}

impl fmt::Display for TavilyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TavilyError::Api(msg) => write!(f, "API error: {}", msg),
            TavilyError::Http(err) => write!(f, "HTTP error: {}", err),
            TavilyError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            TavilyError::RateLimit(msg) => write!(f, "Rate limit error: {}", msg),
        }
    }
}

impl From<reqwest::Error> for TavilyError {
    fn from(err: reqwest::Error) -> Self {
        TavilyError::Http(err)
    }
}
