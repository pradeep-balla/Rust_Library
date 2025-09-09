use reqwest;
use tokio::runtime::Runtime;

pub fn make_http_request(
    method: String,
    url: String,
    body: String,
    signature_b64: Option<String>, // Option for signature
    thumbprint: Option<&str>,      // Option for thumbprint
) -> String {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async move {
        let client = reqwest::Client::new();
        let mut req = client
            .request(
                reqwest::Method::from_bytes(method.as_bytes())
                    .unwrap_or(reqwest::Method::POST),
                &url,
            )
            .header("Content-Type", "application/json")
            .body(body);

        // Only add headers if signature exists
        if let Some(sig) = signature_b64 {
            req = req.header("X-Signature", sig);

            if let Some(tp) = thumbprint {
                req = req.header("X-Thumbprint", tp);
            }
        }

        match req.send().await {
            Ok(resp) => resp.text().await.unwrap_or_else(|e| e.to_string()),
            Err(e) => e.to_string(),
        }
    })
}

