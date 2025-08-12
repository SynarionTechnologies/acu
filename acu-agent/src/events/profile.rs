use serde::{Deserialize, Serialize};

use super::EventMeta;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProfileCreated {
    pub meta: EventMeta,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProfileRenamed {
    pub meta: EventMeta,
    pub new_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PolicyUpdated {
    pub meta: EventMeta,
    pub policy: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigUpdated {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AuthenticationKeyAdded {
    pub meta: EventMeta,
    pub key_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AuthenticationKeyRevoked {
    pub meta: EventMeta,
    pub key_id: String,
}
