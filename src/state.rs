use crate::services::{expo::ExpoService, repeater::WebhookRepeaterService};

pub struct AppState {
    pub expo: ExpoService,
    pub repeater: WebhookRepeaterService,
    pub http_client: reqwest::Client,
}
