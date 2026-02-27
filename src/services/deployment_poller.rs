use std::{env, sync::Arc, time::Duration};

use crate::services::expo::{ExpoNotification, ExpoService};
use crate::state::AppState;

pub struct DeploymentPollerService {
    pub api_url: String,
    pub api_token: String,
    pub poll_interval_secs: u64,
    pub api_endpoint: String,
    client: reqwest::Client,
    deployments: Vec<String>,
}

impl DeploymentPollerService {
    pub fn new(
        api_url: String,
        api_token: String,
        api_endpoint: String,
        poll_interval_secs: u64,
    ) -> Self {
        Self {
            api_url,
            api_token,
            api_endpoint,
            poll_interval_secs,
            client: reqwest::Client::new(),
            deployments: Vec::new(),
        }
    }

    fn deployments_url(&self) -> String {
        format!(
            "{}/{}",
            self.api_url.trim_end_matches('/'),
            self.api_endpoint
                .trim_start_matches('/')
                .trim_end_matches('/')
        )
    }

    pub async fn check_for_deployments(&self) -> Result<Vec<serde_json::Value>, String> {
        let response = self
            .client
            .get(self.deployments_url())
            .bearer_auth(&self.api_token)
            .send()
            .await
            .map_err(|e| format!("Failed to call Coolify deployments API: {}", e))?;

        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read Coolify deployments API response: {}", e))?;

        let payload: Vec<serde_json::Value> = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse Coolify deployments API response: {}", e))?;

        Ok(payload)
    }

    pub async fn send_notification_to_device(
        &mut self,
        expo: &ExpoService,
        payload: Vec<serde_json::Value>,
    ) {
        if payload.is_empty() {
            self.deployments.clear();
            return;
        }

        let current_uuids: Vec<String> = payload
            .iter()
            .filter_map(|d| d.get("deployment_uuid").and_then(|v| v.as_str()))
            .map(String::from)
            .collect();
        self.deployments.retain(|uuid| current_uuids.contains(uuid));

        for deployment in payload {
            let Some(deployment_uuid) = deployment
                .get("deployment_uuid")
                .and_then(|v| v.as_str())
                .map(String::from)
            else {
                continue;
            };

            if self.deployments.contains(&deployment_uuid) {
                continue;
            }

            self.deployments.push(deployment_uuid);

            let application_name = deployment
                .get("application_name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            expo.send_notification(ExpoNotification {
                title: "Deployment Started".to_string(),
                body: format!("New deployment has started for {}", application_name),
                data: deployment,
            })
            .await;
        }
    }

    pub fn from_env() -> Result<Option<Self>, &'static str> {
        let api_url = env::var("COOLIFY_API_URL")
            .ok()
            .map(|url| url.trim().to_string())
            .filter(|url| !url.is_empty());
        let api_token = env::var("COOLIFY_API_TOKEN")
            .ok()
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty());
        let poll_interval_secs = env::var("COOLIFY_DEPLOYMENT_POLL_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(10);
        let api_endpoint = env::var("COOLIFY_API_ENDPOINT")
            .ok()
            .map(|endpoint| endpoint.trim().to_string())
            .filter(|endpoint| !endpoint.is_empty())
            .unwrap_or("api/v1/deployments".to_string());

        match (api_url, api_token, api_endpoint) {
            (None, _, _) => Ok(None),
            (Some(_), None, _) => Err(
                "Environment variable COOLIFY_API_TOKEN is required when COOLIFY_API_URL is set.",
            ),
            (Some(api_url), Some(api_token), api_endpoint) => Ok(Some(Self::new(
                api_url,
                api_token,
                api_endpoint,
                poll_interval_secs,
            ))),
        }
    }

    pub fn start_polling(state: Arc<AppState>) -> Result<(), &'static str> {
        let Some(mut deployment_poller) = Self::from_env()? else {
            return Ok(());
        };

        println!("Deployment poller initialized");

        tokio::spawn(async move {
            loop {
                match deployment_poller.check_for_deployments().await {
                    Ok(payload) => {
                        deployment_poller
                            .send_notification_to_device(&state.expo, payload)
                            .await;
                    }
                    Err(error) => eprintln!("{}", error),
                }

                tokio::time::sleep(Duration::from_secs(deployment_poller.poll_interval_secs)).await;
            }
        });

        Ok(())
    }
}
