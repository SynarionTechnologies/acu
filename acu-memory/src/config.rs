use serde::Deserialize;

/// Weights for computing the retention score of a memory item.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RetentionWeights {
    pub recency: f32,
    pub reward: f32,
    #[serde(rename = "reference_count")]
    pub reference_count: f32,
    pub uniqueness: f32,
}

/// Retention policy configuration loaded from TOML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RetentionConfig {
    pub episodic_ttl_days: i64,
    pub tool_logs_ttl_days: i64,
    pub purge_threshold: f32,
    pub weights: RetentionWeights,
}

impl RetentionConfig {
    /// Parses the retention configuration from a TOML string.
    pub fn from_toml_str(toml_str: &str) -> Result<Self, toml::de::Error> {
        #[derive(Deserialize)]
        struct Root {
            memory: MemorySection,
        }
        #[derive(Deserialize)]
        struct MemorySection {
            retention: RetentionConfig,
        }
        let root: Root = toml::from_str(toml_str)?;
        Ok(root.memory.retention)
    }

    /// Returns the TTL in days for the given channel.
    pub fn ttl_for_channel(&self, channel: &super::MemoryChannel) -> i64 {
        match channel {
            super::MemoryChannel::Episodic => self.episodic_ttl_days,
            super::MemoryChannel::ToolLogs => self.tool_logs_ttl_days,
            // default TTL for channels without explicit configuration
            _ => 365,
        }
    }
}

