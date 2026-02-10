use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{env, time::{Duration, SystemTime}};

pub fn parse_expo_push_tokens() -> Result<Vec<String>, String> {
    let expo_push_tokens = env::var("EXPO_PUSH_TOKENS");

    let Ok(expo_push_tokens) = &expo_push_tokens else {
        return Err("NOT_SET".to_string());
    };

    let mut valid_tokens = Vec::new();
    let tokens = &expo_push_tokens.split(',').map(str::to_string).collect::<Vec<String>>();
    let re = Regex::new(r"ExponentPushToken\[(?<token>[^\]]+)\]");

    let Ok(re) = &re else {
        return Err("Failed to create regex.".to_string());
    };

    for token in tokens {
        if !&re.is_match(&token.as_str()) {
            eprintln!("Invalid Expo push token: {}", token);
            eprintln!("Please use the correct format: ExponentPushToken[1234567890]");
            continue;
        }

        valid_tokens.push(token.to_string());
    }

    if valid_tokens.is_empty() {
        eprintln!("No valid Expo push tokens found.");
        return Err("No valid Expo push tokens found.".to_string());
    }

    Ok(valid_tokens)
}

#[derive(Debug, Serialize)]
pub struct ExpoNotification {
    to: String,
    title: String,
    body: String,
    data: serde_json::Value,
}

impl ExpoNotification {
    pub fn new(to: String, title: String, body: String, data: serde_json::Value) -> Self {
        Self {
            to,
            title,
            body,
            data,
        }
    }
}

pub fn send_expo_notification(expo_notifications: Vec<ExpoNotification>) {
    let expo_push_url = env::var("EXPO_PUSH_URL")
        .unwrap_or_else(|_| "https://exp.host/--/api/v2/push/send".to_string());
    
    let client = reqwest::Client::new();

    tokio::spawn(async move {
        let json_body = serde_json::to_string(&expo_notifications);

        match json_body {
            Ok(body) => {
                let res = client
                    .post(&expo_push_url)
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await;

                match res {
                    Ok(response) => println!("Expo response: {:?}", response.text().await.unwrap()),
                    Err(e) => eprintln!("Failed to send Expo notification: {:?}", e),
                }
            }
            Err(e) => eprintln!("Failed to serialize Expo notification: {:?}", e),
        }
    });
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
}
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const CHECK_FOR_UPDATES_INTERVAL: u64 = 60 * 60 * 24;

static mut UPDATE_NOTIFICATION_SENT: bool = false;
static mut LAST_CHECK_TIME: Option<SystemTime> = None;

pub fn check_for_updates(expo_push_tokens: Vec<String>) {
    if unsafe { UPDATE_NOTIFICATION_SENT } {
        return;
    }

    if let Some(last_check_time) = unsafe { LAST_CHECK_TIME } {
        if last_check_time.elapsed().unwrap() < Duration::from_secs(CHECK_FOR_UPDATES_INTERVAL) {
            return;
        }
    }

    unsafe { LAST_CHECK_TIME = Some(SystemTime::now()); }

    let client = reqwest::Client::new();
    tokio::spawn(async move {
        if let Ok(res) = client
            .get("https://api.github.com/repos/jacxk/coolify-expo-notification-relay/releases/latest")
            .header("User-Agent", format!("{} v{}", PACKAGE_NAME, VERSION))
            .send()
            .await 
        {
            let body = res.text().await.ok();
            let release = body
                .as_ref()
                .and_then(|b| serde_json::from_str::<Release>(b).ok());
            if let Some(release) = release {
                unsafe { UPDATE_NOTIFICATION_SENT = true; }
                
                if release.tag_name != format!("v{}", VERSION) {
                    send_expo_notification(expo_push_tokens.iter().map(|token| 
                        ExpoNotification::new(
                            token.to_string(),
                            "Update Available".to_string(),
                            format!("New update available for {} is available", PACKAGE_NAME),
                            serde_json::json!({
                                "latest_version": release.tag_name,
                                "current_version": VERSION,
                                "release_url": release.html_url,
                            }),
                        )).collect::<Vec<ExpoNotification>>()
                    );

                    println!("-------------------------------------------------");
                    println!("Latest version: {}", release.tag_name);
                    println!("Current version: {}", VERSION);
                    println!("If running in docker, you can update by running: docker pull ghcr.io/jacxk/coolify-expo-notification-relay:latest");
                    println!("If running on coolify, you can redeploy the application.");
                    println!("-------------------------------------------------");
                }
            }
        }
    });
}
