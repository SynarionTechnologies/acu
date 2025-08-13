use super::{ExplorationResult, ExplorationStrategy};
use crate::{errors::Result, ExplorationPlan};

#[derive(Default)]
pub struct ApiQueryStrategy;

impl ExplorationStrategy for ApiQueryStrategy {
    fn execute(&self, plan: &ExplorationPlan) -> Result<ExplorationResult> {
        Ok(ExplorationResult {
            content: format!("api response for {}", plan.target),
        })
    }
}
