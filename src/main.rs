mod endpoints;
mod event_parser;
mod utils;

use crate::endpoints::health_check;
use crate::endpoints::webhook_catch;
use crate::utils::check_for_updates;
use crate::utils::parse_expo_push_tokens;

use axum::{
    Router,
    routing::{get, post},
};
use std::env;

#[tokio::main]
async fn main() {
    let expo_push_tokens = parse_expo_push_tokens();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let webhook_path = env::var("WEBHOOK_PATH").unwrap_or_else(|_| "/".to_string());

    check_for_updates(expo_push_tokens.clone());
    let app = Router::new()
        .route("/health", get(health_check::health_check))
        .route(
            webhook_path.as_str(),
            post(move |payload| {
                webhook_catch::webhook_catch(payload, expo_push_tokens)
            }),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Relay server is running");
    println!("Health check: http://localhost:{}/health", port);
    println!("Webhook: http://localhost:{}{}", port, webhook_path);

    axum::serve(listener, app).await.unwrap();
}
