pub struct WebhookRepeaterService {
    pub urls: Vec<String>,
    pub client: reqwest::Client,
}

impl WebhookRepeaterService {
    const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    pub async fn forward(&self, payload: &serde_json::Value) -> Result<(), &str> {
        let Ok(body) = serde_json::to_string(payload) else {
            return Err("Failed to serialize payload.");
        };

        for url in self.urls.iter() {
            let result = self
                .client
                .post(url)
                .body(body.clone())
                .header("Content-Type", "application/json")
                .header(
                    "User-Agent",
                    format!(
                        "{} v{}",
                        WebhookRepeaterService::PACKAGE_NAME,
                        WebhookRepeaterService::VERSION
                    ),
                )
                .send()
                .await;

            match result {
                Err(e) => eprintln!("Failed to forward webhook to repeater: {}", e),
                _ => (),
            }
        }

        Ok(())
    }
}
