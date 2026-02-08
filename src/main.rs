use std::env;
use axum::{
    Json, Router, routing::{get, post}
};
use regex::Regex;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let webhook_path = env::var("WEBHOOK_PATH").unwrap_or_else(|_| "/".to_string());
    let expo_push_tokens = parse_expo_push_tokens();

    let app = Router::new()
    .route("/health", get(health_check))
    .route(webhook_path.as_str(), post(webhook));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("Relay server is running");
    println!("Health check: http://localhost:{}/health", port);
    println!("Webhook: http://localhost:{}{}", port, webhook_path);
    axum::serve(listener, app).await.unwrap();
}

fn parse_expo_push_tokens() -> Vec<String> {
    let expo_push_tokens = env::var("EXPO_PUSH_TOKENS");

    if expo_push_tokens.is_err() {
        eprintln!("Environment variable EXPO_PUSH_TOKENS is not set.");
        eprintln!("Please set the environment variable and try again.");
        eprintln!("-------------------------------------------------");
        eprintln!("Example: EXPO_PUSH_TOKENS='ExponentPushToken[1234567890]'");
        eprintln!("For multiple tokens, use a comma-separated list: EXPO_PUSH_TOKENS='ExponentPushToken[1234567890],ExponentPushToken[1234567891]'");
        eprintln!();
        eprintln!("You can find your Expo push tokens in the app settings.");
        eprintln!("-------------------------------------------------");
        std::process::exit(1);
    }
    
    let tokens: Vec<String> = expo_push_tokens.unwrap().split(',').map(|token| token.to_string()).collect();
    let re = Regex::new(r"ExponentPushToken\[(?<token>[^\]]+)\]").unwrap();
    let mut valid_tokens = Vec::new();

    for token in tokens {
        if !re.is_match(token.as_str()) {
            eprintln!("Invalid Expo push token: {}", token);
            eprintln!("Please use the correct format: ExponentPushToken[1234567890]");
            continue;
        }

        valid_tokens.push(token);
    }

    if valid_tokens.is_empty() {
        eprintln!("No valid Expo push tokens found.");
        std::process::exit(1);
    }

    valid_tokens
}

async fn health_check() -> &'static str {
    "OK"
}

async fn webhook(payload: Json<serde_json::Value>) -> &'static str {
    println!("Webhook received: {:?}", payload);
    "OK"
}