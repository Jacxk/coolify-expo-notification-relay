use std::env;

use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::utils::ExpoNotification;
use crate::{
    event_parser,
    utils::{check_for_updates, send_expo_notification},
};

pub async fn webhook_catch(
    payload: Json<serde_json::Value>,
    expo_push_tokens: Vec<String>,
) -> impl IntoResponse {
    relay_payload(&payload);
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

    (StatusCode::OK, "OK").into_response()
}

fn relay_payload(payload: &Json<serde_json::Value>) {
    let relay_urls = env::var("WEBHOOK_RELAY_URLS")
        .unwrap_or("".to_string())
        .split(',')
        .map(|url| url.trim().to_string())
        .filter(|url| !url.is_empty())
        .collect::<Vec<String>>();

    let client = reqwest::Client::new();
    let payload_clone = payload.0.clone();

    tokio::spawn(async move {
        for url in relay_urls {
            let result = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&payload_clone).unwrap())
                .send()
                .await;

            match result {
                Ok(response) => match response.text().await {
                    Ok(text) => println!("Relay response from {}: {:?}", url, text),
                    Err(e) => {
                        eprintln!("Failed to read relay response from {}: {:?}", url, e)
                    }
                },
                Err(err) => {
                    eprintln!("Failed to relay payload to {}: {:?}", url, err);
                }
            }
        }
    });
}
