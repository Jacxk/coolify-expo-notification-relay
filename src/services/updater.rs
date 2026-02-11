use std::time::{Duration, SystemTime};

use coolify_expo_notification_relay::event_parser::WebhookPayload;
use serde::Deserialize;

use crate::services::expo::{ExpoNotification, ExpoService};

pub struct UpdaterService {
    pub current_version: &'static str,
    pub release: Option<Release>,
    pub update_available: bool,
    pub notification_sent: bool,
    pub last_check_time: Option<SystemTime>,
    pub check_for_updates_interval: u64,
    pub update_check_url: &'static str,
    client: reqwest::Client,
}

#[derive(Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub html_url: String,
}

#[derive(Debug)]
pub struct UpdaterError {
    pub message: &'static str,
}

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Default for UpdaterService {
    fn default() -> Self {
        Self {
            current_version: VERSION,
            update_check_url: "https://api.github.com/repos/jacxk/coolify-expo-notification-relay/releases/latest",
            release: None,
            update_available: false,
            notification_sent: false,
            last_check_time: None,
            check_for_updates_interval: 86400,
            client: reqwest::Client::new(),
        }
    }
}

impl UpdaterService {
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client,
            ..Default::default()
        }
    }

    pub async fn check_for_updates(&mut self) -> Result<Option<Release>, UpdaterError> {
        if let Some(last_check_time) = self.last_check_time {
            if let Ok(elapsed) = last_check_time.elapsed() {
                if elapsed < Duration::from_secs(self.check_for_updates_interval) {
                    return Ok(None);
                }
            } else {
                return Err(UpdaterError {
                    message: "Failed to get elapsed time.",
                });
            }
        }

        let Ok(res) = self.client
            .get(self.update_check_url)
            .header(
                "User-Agent",
                format!("{} v{}", PACKAGE_NAME, self.current_version),
            )
            .send()
            .await
        else {
            return Err(UpdaterError {
                message: "Failed to send request to Github API.",
            });
        };

        let Ok(body) = res.text().await else {
            return Err(UpdaterError {
                message: "Failed to get body from Github API.",
            });
        };
        let release = serde_json::from_str::<Release>(&body);
        let Ok(release) = release else {
            return Err(UpdaterError {
                message: "Failed to parse release from Github API.",
            });
        };

        self.last_check_time = Some(SystemTime::now());
        self.release = Some(release.clone());

        if release.tag_name != format!("v{}", self.current_version) {
            self.update_available = true;
            Ok(Some(release))
        } else {
            Ok(None)
        }
    }

    pub async fn send_notification_to_device(&mut self, expo: &ExpoService) {
        let notification = ExpoNotification {
            title: "Update Available".to_string(),
            body: format!(
                "A new version of the {} is available. Current version: v{}, latest version: {}",
                PACKAGE_NAME,
                self.current_version,
                self.release.as_ref().unwrap().tag_name
            ),
            data: WebhookPayload::default(),
        };
        expo.send_notification(notification).await;
        self.notification_sent = true;
    }
}
