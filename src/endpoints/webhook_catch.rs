use std::env;

use axum::Json;

use crate::utils::ExpoNotification;
use crate::{
    event_parser,
    utils::{check_for_updates, send_expo_notification},
};

pub async fn webhook_catch(
    payload: Json<serde_json::Value>,
    expo_push_tokens: Vec<String>,
) -> &'static str {
    relay_payload(payload.clone());
    check_for_updates(expo_push_tokens.clone());

    let data = payload.0.clone();
    let serialized_data = serde_json::from_value(data.clone()).unwrap();
    let notification = event_parser::parse_event(&serialized_data);
    let expo_notifications = expo_push_tokens
        .iter()
        .map(|token| {
            ExpoNotification::new(
                token.clone(),
                notification.title.to_string(),
                notification.body.to_string(),
                data.clone(),
            )
        })
        .collect::<Vec<ExpoNotification>>();

    println!(
        "Webhook Event Received: {:?}",
        serialized_data.event.as_deref().unwrap_or("unknown event")
    );

    send_expo_notification(expo_notifications);

    "OK"
}

fn relay_payload(payload: Json<serde_json::Value>) {
    let relay_urls = env::var("WEBHOOK_RELAY_URLS")
        .unwrap_or_else(|_| "".to_string())
        .split(',')
        .map(|url| url.trim().to_string())
        .filter(|url| !url.is_empty())
        .collect::<Vec<String>>();

    for url in relay_urls {
        let url_clone = url.clone();
        let payload_clone = payload.0.clone();

        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let result = client
                .post(&url_clone)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&payload_clone).unwrap())
                .send()
                .await;

            match result {
                Ok(response) => match response.text().await {
                    Ok(text) => println!("Relay response from {}: {:?}", url_clone, text),
                    Err(e) => {
                        eprintln!("Failed to read relay response from {}: {:?}", url_clone, e)
                    }
                },
                Err(err) => {
                    eprintln!("Failed to relay payload to {}: {:?}", url_clone, err);
                }
            }
        });
    }
}
