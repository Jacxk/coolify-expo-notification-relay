mod services;
mod state;

use coolify_expo_notification_relay::utils::parse_expo_push_tokens;

use axum::{
    Router,
    routing::{get, post},
};
use reqwest::StatusCode;
use std::env;
use std::sync::Arc;

use crate::services::{
    expo::ExpoService, repeater::WebhookRepeaterService, updater::UpdaterService,
};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let Ok(expo_push_tokens) = parse_expo_push_tokens() else {
        eprintln!("Environment variable EXPO_PUSH_TOKENS is not set.");
        eprintln!("Please set the environment variable and try again.");
        eprintln!("-------------------------------------------------");
        eprintln!("Example: EXPO_PUSH_TOKENS='ExponentPushToken[1234567890]'");
        eprintln!(
            "For multiple tokens, use a comma-separated list: EXPO_PUSH_TOKENS='ExponentPushToken[1234567890],ExponentPushToken[1234567891]'"
        );
        eprintln!();
        eprintln!("You can find your Expo push tokens in the app settings.");
        eprintln!("-------------------------------------------------");
        std::process::exit(1)
    };

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let webhook_path = env::var("WEBHOOK_PATH").unwrap_or_else(|_| "/".to_string());
    let expo_push_url = env::var("EXPO_PUSH_URL")
        .unwrap_or_else(|_| "https://exp.host/--/api/v2/push/send".to_string());

    let state = Arc::new(AppState {
        expo: ExpoService::new(expo_push_tokens.clone(), expo_push_url),
        repeater: WebhookRepeaterService {
            urls: env::var("WEBHOOK_RELAY_URLS")
                .unwrap_or("".to_string())
                .split(',')
                .map(|url| url.trim().to_string())
                .filter(|url| !url.is_empty())
                .collect::<Vec<String>>(),
        },
        expo_push_tokens,
    });

    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut updater = UpdaterService::default();
        if let Ok(Some(release)) = updater.check_for_updates().await {
            println!("-------------------------------------------------");
            println!("Latest version: {}", release.tag_name);
            println!("Current version: {}", updater.current_version);
            println!(
                "If running in docker, you can update by running: docker pull ghcr.io/jacxk/coolify-expo-notification-relay:latest"
            );
            println!("If running on coolify, you can redeploy the application.");
            println!("-------------------------------------------------");

            updater.send_notification_to_device(&state_clone.expo).await;
        }
    });

    let app = Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .route(webhook_path.as_str(), post(services::handle_webhook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to bind to port {}: {}", port, e);
            std::process::exit(1);
        });

    println!("Relay server is running");
    println!("Health check: http://localhost:{}/health", port);
    println!("Webhook: http://localhost:{}{}", port, webhook_path);

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("Failed to serve application: {}", e);
        std::process::exit(1);
    });
}
