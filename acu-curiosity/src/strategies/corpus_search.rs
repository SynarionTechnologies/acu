use super::{ExplorationResult, ExplorationStrategy};
use crate::{errors::Result, ExplorationPlan};

#[derive(Default)]
pub struct CorpusSearchStrategy;

impl ExplorationStrategy for CorpusSearchStrategy {
    fn execute(&self, plan: &ExplorationPlan) -> Result<ExplorationResult> {
        Ok(ExplorationResult {
            content: format!("search results for {}", plan.target),
        })
    }
}
