use reqwest;
use tokio::runtime::Runtime;

pub fn make_http_request(
    method: String,
    url: String,
    body: String,
    signature_b64: String,
    thumbprint: &str,
) -> String {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async move {
        let client = reqwest::Client::new();
        let req = client
            .request(reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::POST), &url)
            .header("Content-Type", "application/json")
            .header("X-Signature", signature_b64)
            .header("X-Thumbprint", thumbprint)
            .body(body);

        match req.send().await {
            Ok(resp) => resp.text().await.unwrap_or_else(|e| e.to_string()),
            Err(e) => e.to_string(),
        }
    })
}
