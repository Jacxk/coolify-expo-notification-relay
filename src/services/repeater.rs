use crate::WebhookPayload;

pub struct WebhookRepeaterService {
    pub urls: Vec<String>,
    pub client: reqwest::Client,
}

impl WebhookRepeaterService {
    pub fn forward(&self, payload: &WebhookPayload) -> Result<(), &str> {
        let Ok(body) = serde_json::to_string(payload) else {
            return Err("Failed to serialize payload.");
        };

        for url in self.urls.iter() {
            let _ = self
                .client
                .post(url)
                .body(body.clone())
                .header("Content-Type", "application/json")
                .header("User-Agent", "Coolify Expo Notification Relay")
                .send();
        }

        Ok(())
    }
}
