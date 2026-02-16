use std::{env, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};

use crate::services::expo::{ExpoNotification, ExpoService};
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    application_id: String,
    application_name: String,
    deployment_uuid: String,
    id: u64,
}

pub struct DeploymentPollerService {
    pub api_url: String,
    pub api_token: String,
    pub poll_interval_secs: u64,
    client: reqwest::Client,
    deployments: Vec<String>,
}

impl DeploymentPollerService {
    pub fn new(api_url: String, api_token: String, poll_interval_secs: u64) -> Self {
        Self {
            api_url,
            api_token,
            poll_interval_secs,
            client: reqwest::Client::new(),
            deployments: Vec::new(),
        }
    }

    fn deployments_url(&self) -> String {
        format!("{}/api/v1/deployments", self.api_url.trim_end_matches('/'))
    }

    pub async fn check_for_deployments(&self) -> Result<Vec<Deployment>, String> {
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

        let payload: Vec<Deployment> = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse Coolify deployments API response: {}", e))?;

        Ok(payload)
    }

    pub async fn send_notification_to_device(
        &mut self,
        expo: &ExpoService,
        payload: Vec<Deployment>,
    ) {
        if payload.len() == 0 {
            self.deployments.clear();
            return;
        }

        let current_uuids = payload
            .iter()
            .map(|d| d.deployment_uuid.clone())
            .collect::<Vec<String>>();
        self.deployments.retain(|uuid| current_uuids.contains(uuid));

        for deployment in payload {
            let deployment_uuid = &deployment.deployment_uuid;
            if self.deployments.contains(deployment_uuid) {
                continue;
            }

            self.deployments.push(deployment_uuid.clone());

            expo.send_notification(ExpoNotification {
                title: "Deployment Started".to_string(),
                body: format!(
                    "New deployment has started for {}",
                    deployment.application_name
                ),
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

        match (api_url, api_token) {
            (None, _) => Ok(None),
            (Some(_), None) => Err(
                "Environment variable COOLIFY_API_TOKEN is required when COOLIFY_API_URL is set.",
            ),
            (Some(api_url), Some(api_token)) => {
                Ok(Some(Self::new(api_url, api_token, poll_interval_secs)))
            }
        }
    }

    pub fn start_polling(state: Arc<AppState>) -> Result<(), &'static str> {
        let Some(mut deployment_poller) = Self::from_env()? else {
            return Ok(());
        };

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
