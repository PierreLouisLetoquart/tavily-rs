use std::fmt;
use std::time::Duration;

use serde_json::Value;

pub type Result<T> = core::result::Result<T, TavilyError>;

/// Errors returned by the Tavily client.
///
/// Error responses from the API are split by cause, each carrying the [`ApiError`] details:
///
/// ```no_run
/// # async fn run(tavily: tavily::Tavily) {
/// use tavily::TavilyError;
///
/// match tavily.search("rust").await {
///     Ok(response) => println!("{} results", response.results.len()),
///     Err(TavilyError::Unauthorized(_)) => eprintln!("check your API key"),
///     Err(TavilyError::RateLimited { retry_after, .. }) => eprintln!("retry in {retry_after:?}"),
///     Err(TavilyError::UsageLimit(err)) => eprintln!("out of credits: {}", err.message),
///     Err(err) => eprintln!("{err}"),
/// }
/// # }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum TavilyError {
    /// The API key is missing or invalid (401).
    Unauthorized(ApiError),
    /// The request was rejected (400 or 422), e.g. an invalid parameter or parameter
    /// combination.
    BadRequest(ApiError),
    /// Too many requests (429), still failing after all retries. `retry_after` is the delay
    /// requested by Tavily in the last response, if any.
    RateLimited {
        error: ApiError,
        retry_after: Option<Duration>,
    },
    /// The key, plan or pay-as-you-go usage limit was exceeded (432 or 433).
    UsageLimit(ApiError),
    /// Any other error response, e.g. a server error still failing after all retries.
    Api(ApiError),
    /// The request could not be sent or its response could not be read or decoded.
    Http(reqwest::Error),
    /// The client is misconfigured, e.g. no API key.
    Configuration(String),
}

impl TavilyError {
    /// The HTTP status of the error response, if the API returned one.
    pub fn status(&self) -> Option<u16> {
        self.api_error().map(|error| error.status)
    }

    /// The details of the error response, if the API returned one.
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            TavilyError::Unauthorized(error)
            | TavilyError::BadRequest(error)
            | TavilyError::RateLimited { error, .. }
            | TavilyError::UsageLimit(error)
            | TavilyError::Api(error) => Some(error),
            TavilyError::Http(_) | TavilyError::Configuration(_) => None,
        }
    }

    /// Build the error matching an error response.
    pub(crate) fn from_response(status: u16, body: String, retry_after: Option<Duration>) -> Self {
        let error = ApiError::new(status, body);
        match status {
            401 => TavilyError::Unauthorized(error),
            400 | 422 => TavilyError::BadRequest(error),
            429 => TavilyError::RateLimited { error, retry_after },
            432 | 433 => TavilyError::UsageLimit(error),
            _ => TavilyError::Api(error),
        }
    }
}

/// An error response from the Tavily API.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ApiError {
    /// The HTTP status code.
    pub status: u16,
    /// The error message sent by Tavily, or the raw body when it has none.
    pub message: String,
    /// The raw response body.
    pub body: String,
}

impl ApiError {
    fn new(status: u16, body: String) -> Self {
        let message = parse_message(&body).unwrap_or_else(|| body.trim().to_string());
        Self {
            status,
            message,
            body,
        }
    }
}

/// Read the message from Tavily's error bodies: `{"detail": {"error": "..."}}`,
/// `{"detail": "..."}` or, for validation errors (422), `{"detail": [{"loc": [...], "msg": "..."}]}`.
fn parse_message(body: &str) -> Option<String> {
    let detail = serde_json::from_str::<Value>(body)
        .ok()?
        .get("detail")?
        .clone();
    match detail {
        Value::String(message) => Some(message),
        Value::Object(object) => object.get("error")?.as_str().map(str::to_string),
        Value::Array(items) => {
            let messages: Vec<String> = items
                .iter()
                .filter_map(|item| {
                    let msg = item.get("msg")?.as_str()?;
                    let loc = item.get("loc").and_then(Value::as_array).map(|loc| {
                        loc.iter()
                            .map(|part| match part {
                                Value::String(part) => part.clone(),
                                other => other.to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(".")
                    });
                    Some(match loc {
                        Some(loc) if !loc.is_empty() => format!("{loc}: {msg}"),
                        _ => msg.to_string(),
                    })
                })
                .collect();
            (!messages.is_empty()).then(|| messages.join("; "))
        }
        _ => None,
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl std::error::Error for ApiError {}

impl fmt::Display for TavilyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Tavily's messages already describe the cause, so API errors share one format.
            TavilyError::Unauthorized(error)
            | TavilyError::BadRequest(error)
            | TavilyError::RateLimited { error, .. }
            | TavilyError::UsageLimit(error)
            | TavilyError::Api(error) => write!(f, "Tavily API error {}", error),
            TavilyError::Http(err) => write!(f, "HTTP error: {}", err),
            TavilyError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for TavilyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TavilyError::Http(err) => Some(err),
            _ => self.api_error().map(|error| error as _),
        }
    }
}

impl From<reqwest::Error> for TavilyError {
    fn from(err: reqwest::Error) -> Self {
        TavilyError::Http(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_status_codes_to_variants() {
        let error = |status| TavilyError::from_response(status, String::new(), None);

        assert!(matches!(error(401), TavilyError::Unauthorized(_)));
        assert!(matches!(error(400), TavilyError::BadRequest(_)));
        assert!(matches!(error(422), TavilyError::BadRequest(_)));
        assert!(matches!(error(429), TavilyError::RateLimited { .. }));
        assert!(matches!(error(432), TavilyError::UsageLimit(_)));
        assert!(matches!(error(433), TavilyError::UsageLimit(_)));
        assert!(matches!(error(434), TavilyError::Api(_)));
        assert!(matches!(error(500), TavilyError::Api(_)));
        assert_eq!(error(432).status(), Some(432));
    }

    #[test]
    fn reads_tavily_error_messages() {
        let message = |body: &str| ApiError::new(400, body.to_string()).message;

        assert_eq!(
            message(r#"{"detail":{"error":"Invalid topic."}}"#),
            "Invalid topic."
        );
        assert_eq!(message(r#"{"detail":"Not found"}"#), "Not found");
        assert_eq!(
            message(
                r#"{"detail":[{"type":"string_type","loc":["body","query"],"msg":"Input should be a valid string","input":[]}]}"#
            ),
            "body.query: Input should be a valid string"
        );
        assert_eq!(message("  upstream timeout \n"), "upstream timeout");
    }

    #[test]
    fn displays_status_and_message() {
        let error = TavilyError::from_response(
            401,
            r#"{"detail":{"error":"Unauthorized: missing or invalid API key."}}"#.into(),
            None,
        );

        assert_eq!(
            error.to_string(),
            "Tavily API error 401: Unauthorized: missing or invalid API key."
        );
    }
}
