use crate::errors::Result;
use crate::ExplorationPlan;

pub trait ExplorationStrategy {
    fn execute(&self, plan: &ExplorationPlan) -> Result<ExplorationResult>;
}

#[derive(Debug, Clone)]
pub struct ExplorationResult {
    pub content: String,
}

pub mod api_query;
pub mod corpus_search;
pub mod web_crawl;
