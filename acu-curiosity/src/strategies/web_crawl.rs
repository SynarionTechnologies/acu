use super::{ExplorationResult, ExplorationStrategy};
use crate::{errors::Result, ExplorationPlan};

#[derive(Default)]
pub struct WebCrawlStrategy;

impl ExplorationStrategy for WebCrawlStrategy {
    fn execute(&self, plan: &ExplorationPlan) -> Result<ExplorationResult> {
        Ok(ExplorationResult {
            content: format!("crawled {}", plan.target),
        })
    }
}
