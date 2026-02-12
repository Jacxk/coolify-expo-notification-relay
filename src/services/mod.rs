pub mod expo;
pub mod repeater;
pub mod updater;

use axum::{Json, extract::State, response::IntoResponse};

use reqwest::StatusCode;
use std::sync::Arc;

use crate::{
    WebhookPayload,
    event_parser::{self},
    services::expo::ExpoNotification,
    state::AppState,
};

pub async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    println!(
        "Received event: {}",
        payload.event.as_deref().unwrap_or("unknown event")
    );

    let notification = event_parser::parse_event(&payload);

    tokio::spawn(async move {
        let result = state.repeater.forward(&payload).await;

        match result {
            Ok(()) => println!("Forwarded webhook to repeaters"),
            Err(e) => eprintln!("Failed to forward webhook to repeaters: {}", e),
        }

        state
            .expo
            .send_notification(ExpoNotification {
                title: notification.title,
                body: notification.body,
                data: payload,
            })
            .await;
    });

    (StatusCode::ACCEPTED, "OK").into_response()
}
