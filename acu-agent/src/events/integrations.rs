use serde::{Deserialize, Serialize};

use super::EventMeta;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExternalApiCalled {
    pub meta: EventMeta,
    pub api: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExternalApiFailed {
    pub meta: EventMeta,
    pub api: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WebhookReceived {
    pub meta: EventMeta,
    pub hook: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WebhookSent {
    pub meta: EventMeta,
    pub hook: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct IntegrationEnabled {
    pub meta: EventMeta,
    pub integration: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct IntegrationDisabled {
    pub meta: EventMeta,
    pub integration: String,
}
