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
