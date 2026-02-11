pub mod expo;
pub mod repeater;
pub mod updater;

use axum::{Json, extract::State, response::IntoResponse};

use reqwest::StatusCode;
use std::sync::Arc;

use crate::{event_parser::{self}, services::expo::ExpoNotification, state::AppState, WebhookPayload};

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
        state.repeater.forward(&payload);
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
