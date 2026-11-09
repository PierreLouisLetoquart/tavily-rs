use serde_json::json;
use tavily::{
    ExtractDepth, ExtractFormat, ExtractRequest, ExtractResult, IncludeAnswer, IncludeRawContent,
    SearchDepth, SearchImageResult, SearchIncludeDomainsMode, SearchRequest, SearchResponse,
    SearchTimeRange, SearchTopic,
};

// Example responses taken from https://docs.tavily.com/documentation/api-reference/endpoint/search
// and https://docs.tavily.com/documentation/api-reference/endpoint/extract

#[test]
fn decodes_full_search_response() {
    let body = json!({
        "query": "Who is Leo Messi?",
        "answer": "Lionel Messi, born in 1987, is an Argentine footballer.",
        "images": [
            { "url": "https://example.com/messi.jpg", "description": "Messi celebrating" }
        ],
        "results": [{
            "title": "Lionel Messi Facts | Britannica",
            "url": "https://www.britannica.com/facts/Lionel-Messi",
            "content": "Lionel Messi, an Argentine footballer...",
            "score": 0.81025416,
            "raw_content": null,
            "published_date": "Tue, 11 Mar 2025 17:00:00 GMT",
            "favicon": "https://britannica.com/favicon.png",
            "images": ["https://britannica.com/messi.jpg"],
            "id": "a3f9c2-04"
        }],
        "auto_parameters": { "topic": "general", "search_depth": "basic" },
        "response_time": 1.67,
        "usage": { "credits": 1 },
        "request_id": "123e4567-e89b-12d3-a456-426614174111"
    });

    let response: SearchResponse = serde_json::from_value(body).unwrap();

    assert_eq!(response.query, "Who is Leo Messi?");
    assert!(response.answer.is_some());
    assert!(matches!(
        &response.images[0],
        SearchImageResult::ImageResult { description: Some(_), .. }
    ));
    let result = &response.results[0];
    assert_eq!(result.id.as_deref(), Some("a3f9c2-04"));
    assert!(matches!(&result.images[0], SearchImageResult::Url(_)));
    assert_eq!(
        response.auto_parameters.unwrap()["search_depth"],
        json!("basic")
    );
    assert_eq!(response.usage.unwrap().credits, 1.0);
    assert!(response.request_id.is_some());
}

#[test]
fn decodes_minimal_search_response() {
    let body = json!({
        "query": "rust",
        "answer": null,
        "images": [],
        "results": [{
            "title": "Rust",
            "url": "https://www.rust-lang.org",
            "content": "A language empowering everyone.",
            "score": 0.9
        }],
        "response_time": 0.5
    });

    let response: SearchResponse = serde_json::from_value(body).unwrap();

    assert!(response.answer.is_none());
    assert!(response.request_id.is_none());
    assert!(response.usage.is_none());
    assert!(response.auto_parameters.is_none());
    let result = &response.results[0];
    assert!(result.id.is_none());
    assert!(result.images.is_empty());
}

#[test]
fn decodes_partial_success_extract_response() {
    let body = json!({
        "results": [{
            "url": "https://example.com/article",
            "title": "Example article",
            "raw_content": "Example extracted article content.",
            "images": []
        }],
        "failed_results": [{
            "url": "https://example.com/unavailable",
            "error": "Failed to retrieve content"
        }],
        "response_time": 0.5,
        "request_id": "123e4567-e89b-12d3-a456-426614174111"
    });

    let response: ExtractResult = serde_json::from_value(body).unwrap();

    assert!(response.results[0].images.is_empty());
    assert_eq!(response.results[0].title.as_deref(), Some("Example article"));
    assert_eq!(response.failed_results[0].url, "https://example.com/unavailable");
}

#[test]
fn decodes_extract_response_with_images_and_usage() {
    let body = json!({
        "results": [{
            "url": "https://en.wikipedia.org/wiki/Artificial_intelligence",
            "raw_content": "Artificial intelligence...",
            "images": ["https://upload.wikimedia.org/ai.png"],
            "favicon": "https://en.wikipedia.org/static/favicon/wikipedia.ico"
        }],
        "response_time": 0.02,
        "usage": { "credits": 0 }
    });

    let response: ExtractResult = serde_json::from_value(body).unwrap();

    assert_eq!(response.results[0].images.len(), 1);
    assert!(response.results[0].title.is_none());
    assert!(response.failed_results.is_empty());
    assert!(response.request_id.is_none());
    assert_eq!(response.usage.unwrap().credits, 0.0);
}

#[test]
fn encodes_default_search_request_with_query_only() {
    let request = SearchRequest::new("rust");

    assert_eq!(serde_json::to_value(&request).unwrap(), json!({ "query": "rust" }));
}

#[test]
fn encodes_search_request_options() {
    let request = SearchRequest::new("rust news")
        .search_depth(SearchDepth::UltraFast)
        .topic(SearchTopic::Finance)
        .time_range(SearchTimeRange::Week)
        .include_domains(["rust-lang.org"])
        .include_domains_mode(SearchIncludeDomainsMode::Prefer)
        .include_answer(IncludeAnswer::Advanced)
        .include_raw_content(IncludeRawContent::Text)
        .max_age_hours(-1);

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "query": "rust news",
            "search_depth": "ultra-fast",
            "topic": "finance",
            "time_range": "week",
            "include_domains": ["rust-lang.org"],
            "include_domains_mode": "prefer",
            "include_answer": "advanced",
            "include_raw_content": "text",
            "max_age_hours": -1
        })
    );
}

#[test]
fn encodes_boolean_include_flags() {
    let request = SearchRequest::new("rust")
        .include_answer(true)
        .include_raw_content(false);

    let value = serde_json::to_value(&request).unwrap();

    assert_eq!(value["include_answer"], json!(true));
    assert_eq!(value["include_raw_content"], json!(false));
}

#[test]
fn encodes_extract_request_options() {
    let request = ExtractRequest::new(["https://example.com"])
        .query("pricing")
        .chunks_per_source(5)
        .extract_depth(ExtractDepth::Advanced)
        .format(ExtractFormat::Text)
        .timeout(12.5);

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "urls": ["https://example.com"],
            "query": "pricing",
            "chunks_per_source": 5,
            "extract_depth": "advanced",
            "format": "text",
            "timeout": 12.5
        })
    );
}
