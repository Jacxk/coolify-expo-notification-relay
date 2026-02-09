use std::env;

use axum::Json;
use serde_json::json;

use crate::event_parser;

pub async fn webhook_catch(
    payload: Json<serde_json::Value>,
    expo_push_tokens: Vec<String>,
    expo_push_url: String,
) -> &'static str {
    let data = payload.0.clone();
    let serialized_data = serde_json::from_value(data.clone()).unwrap();
    let notification = event_parser::parse_event(&serialized_data);
    let expo_notifications = expo_push_tokens
        .iter()
        .map(|token| {
            json!({
                "to": token,
                "title": notification.title.clone(),
                "body": notification.body.clone(),
                "data": &data
            })
        })
        .collect::<Vec<serde_json::Value>>();

    println!(
        "Webhook Event Received: {:?}",
        serialized_data.event.as_deref().unwrap_or("unknown event")
    );

    relay_payload(payload);

    let client = reqwest::Client::new();
    let res = client
        .post(expo_push_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&expo_notifications).unwrap())
        .send()
        .await
        .unwrap();

    println!("Expo response: {:?}", res.text().await.unwrap());
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
