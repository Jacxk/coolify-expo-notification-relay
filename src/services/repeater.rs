use coolify_expo_notification_relay::event_parser::WebhookPayload;

pub struct WebhookRepeaterService {
    pub urls: Vec<String>,
}

impl WebhookRepeaterService {
    pub fn forward(&self, payload: &WebhookPayload) {
        let client = reqwest::Client::new();
        let Ok(body) = serde_json::to_string(payload) else {
            eprintln!("Failed to serialize payload: {:?}", payload);
            return;
        };

        for url in self.urls.iter() {
            println!("Forwarding webhook to {}", url);

            let _ = client
                .post(url)
                .body(body.clone())
                .header("Content-Type", "application/json")
                .header("User-Agent", "Coolify Expo Notification Relay")
                .send();
        }
    }
}
