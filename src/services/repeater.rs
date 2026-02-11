use crate::WebhookPayload;


pub struct WebhookRepeaterService {
    pub urls: Vec<String>,
    pub client: reqwest::Client,
}

impl WebhookRepeaterService {
    pub fn forward(&self, payload: &WebhookPayload) {
        let Ok(body) = serde_json::to_string(payload) else {
            eprintln!("Failed to serialize payload: {:?}", payload);
            return;
        };

        for url in self.urls.iter() {
            println!("Forwarding webhook to {}", url);

            let _ = self.client
                .post(url)
                .body(body.clone())
                .header("Content-Type", "application/json")
                .header("User-Agent", "Coolify Expo Notification Relay")
                .send();
        }
    }
}
