use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tavily::{Tavily, TavilyError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const SEARCH_BODY: &str =
    r#"{"query":"q","answer":null,"images":[],"results":[],"response_time":0.1}"#;

/// Serve the given raw HTTP responses one connection at a time (the last one repeats) and
/// count the requests received.
async fn serve(responses: Vec<String>) -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let hits = Arc::new(AtomicUsize::new(0));
    let counter = hits.clone();

    tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let index = counter.fetch_add(1, Ordering::SeqCst);
            let response = responses[index.min(responses.len() - 1)].clone();
            let mut buffer = [0u8; 8192];
            let _ = socket.read(&mut buffer).await;
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        }
    });

    (base_url, hits)
}

fn response(status: &str, extra_headers: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\n{extra_headers}Content-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn client(base_url: &str, max_retries: u32) -> Tavily {
    Tavily::builder("tvly-test")
        .base_url(base_url)
        .max_retries(max_retries)
        .build()
        .unwrap()
}

#[tokio::test]
async fn retries_server_errors_using_retry_after() {
    let (base_url, hits) = serve(vec![
        response("503 Service Unavailable", "Retry-After: 0\r\n", ""),
        response("500 Internal Server Error", "Retry-After: 0\r\n", ""),
        response("200 OK", "", SEARCH_BODY),
    ])
    .await;

    let result = client(&base_url, 3).search("q").await;

    assert!(result.is_ok(), "{result:?}");
    assert_eq!(hits.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn returns_rate_limit_error_when_retries_are_exhausted() {
    let (base_url, hits) = serve(vec![response(
        "429 Too Many Requests",
        "Retry-After: 0\r\n",
        r#"{"detail":{"error":"slow down"}}"#,
    )])
    .await;

    let result = client(&base_url, 2).search("q").await;

    assert!(matches!(result, Err(TavilyError::RateLimit(ref msg)) if msg.contains("slow down")));
    assert_eq!(hits.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn does_not_retry_client_errors() {
    let (base_url, hits) = serve(vec![response(
        "400 Bad Request",
        "",
        r#"{"detail":{"error":"Invalid topic"}}"#,
    )])
    .await;

    let result = client(&base_url, 3).search("q").await;

    assert!(matches!(result, Err(TavilyError::Api(ref msg)) if msg.contains("Invalid topic")));
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}
