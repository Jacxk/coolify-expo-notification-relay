use axum::Json;
use serde_json::json;

pub async fn webhook_catch(
    payload: Json<serde_json::Value>,
    expo_push_tokens: Vec<String>,
    expo_push_url: String,
) -> &'static str {
    let notifications = expo_push_tokens
        .iter()
        .map(|token| {
            json!({
                "to": token,
                "title": "Test Notification",
                "body": "This is a test notification",
                "data": payload.0.clone()
            })
        })
        .collect::<Vec<serde_json::Value>>();

    let client = reqwest::Client::new();
    let res = client
        .post(expo_push_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&notifications).unwrap())
        .send()
        .await
        .unwrap();

    println!("Expo response: {:?}", res.text().await.unwrap());
    "OK"
}