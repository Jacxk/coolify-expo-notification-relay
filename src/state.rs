use crate::services::{expo::ExpoService, repeater::WebhookRepeaterService};

pub struct AppState {
    pub expo: ExpoService,
    pub repeater: WebhookRepeaterService,
    pub expo_push_tokens: Vec<String>,
}
