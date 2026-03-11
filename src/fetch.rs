use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioExecutor;
use hyper_util::client::legacy::Client;
use hyper_rustls::HttpsConnectorBuilder;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_github_file(repo: &str, path: &str) -> Result<String> {
    // 1. Setup the HTTPS connector (required for GitHub)
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_only()
        .enable_http1()
        .build();

    let client = Client::builder(TokioExecutor::new()).build(https);

    // 2. Construct the raw content URL
    // Format: https://raw.githubusercontent.com/{owner/repo}/master/{path}
    let url = format!("https://raw.githubusercontent.com/{}/master/{}", repo, path);

    // 3. Build the request
    let request = Request::builder()
        .uri(url)
        .header("User-Agent", "my-rust-app") // GitHub requires this
        .body(Empty::<Bytes>::new())?;

    // 4. Execute and handle the response
    let mut res = client.request(request).await?;

    if res.status() != StatusCode::OK {
        return Err(format!("Failed to fetch file: {}", res.status()).into());
    }

    // 5. Aggregate the body bytes into a String
    let body_bytes = res.body_mut().collect().await?.to_bytes();
    let content = String::from_utf8(body_bytes.to_vec())?;

    Ok(content)
}
