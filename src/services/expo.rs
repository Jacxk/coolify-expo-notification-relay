use coolify_expo_notification_relay::event_parser::WebhookPayload;
use serde::{Deserialize, Serialize};
use serde_json::{Error, json};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpoNotification {
    pub title: String,
    pub body: String,
    pub data: WebhookPayload,
}

pub struct ExpoService {
    pub expo_push_tokens: Vec<String>,
    pub expo_push_url: String,
    client: reqwest::Client,
}

impl ExpoNotification {
    pub fn to_json_with_token(&self, token: &str) -> Result<String, Error> {
        let payload = json!({
            "to": token,
            "title": self.title,
            "body": self.body,
            "data": self.data,
        });
        serde_json::to_string(&payload)
    }
}

impl ExpoService {
    pub fn new(expo_push_tokens: Vec<String>, expo_push_url: String, client: reqwest::Client) -> Self {
        Self {
            expo_push_tokens,
            expo_push_url,
            client,
        }
    }

    pub async fn send_notification(&self, notification: ExpoNotification) {
        for token in self.expo_push_tokens.iter() {
            let Ok(body) = notification.to_json_with_token(token) else {
                eprintln!("Failed to serialize notification for token: {:?}", token);
                continue;
            };
            println!("Sending Expo notification to {}: {}", token, body);

            let res = self
                .client
                .post(&self.expo_push_url)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await;

            match res {
                Ok(response) => println!(
                    "Expo response: {:?}",
                    response.text().await.unwrap_or_default()
                ),
                Err(e) => eprintln!("Failed to send Expo notification: {:?}", e),
            }
        }
    }
}
