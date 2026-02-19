use serde::{Deserialize, Serialize};

pub mod event_parser;
pub mod utils;
pub mod services;
pub mod state;

pub use services::expo::ExpoService;
pub use services::deployment_poller::DeploymentPollerService;
pub use services::repeater::WebhookRepeaterService;
pub use services::updater::UpdaterService;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct WebhookPayload {
    pub title: Option<String>,
    pub event: Option<String>,
    pub message: Option<String>,

    pub server_name: Option<String>,
    pub database_name: Option<String>,
    pub total_updates: Option<u64>,
    pub disk_usage: Option<f64>,
    pub threshold: Option<f64>,
    pub preview_fqdn: Option<String>,
    pub application_name: Option<String>,
    pub project: Option<String>,
    pub container_name: Option<String>,
    pub affected_servers_count: Option<u64>,
    pub task_name: Option<String>,
}

impl WebhookPayload {
    pub fn from_value(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    pub title: String,
    pub body: String,
}