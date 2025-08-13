use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CuriosityConfig {
    pub default_strategy: String,
}

impl Default for CuriosityConfig {
    fn default() -> Self {
        Self {
            default_strategy: "web_crawl".to_string(),
        }
    }
}
