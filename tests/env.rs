use tavily::{Tavily, TavilyError};

// Single test on purpose: environment variables are process-wide, and this file runs as its
// own test binary, so nothing else races with it.
#[test]
fn from_env_reads_tavily_api_key() {
    std::env::remove_var("TAVILY_API_KEY");
    assert!(matches!(
        Tavily::from_env(),
        Err(TavilyError::Configuration(_))
    ));

    std::env::set_var("TAVILY_API_KEY", "");
    assert!(matches!(
        Tavily::from_env(),
        Err(TavilyError::Configuration(_))
    ));

    std::env::set_var("TAVILY_API_KEY", "tvly-test");
    assert!(Tavily::from_env().is_ok());
    assert!(Tavily::builder_from_env().unwrap().max_retries(1).build().is_ok());

    std::env::remove_var("TAVILY_API_KEY");
}
