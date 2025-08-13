use super::{AnalysisOutput, Analyzer};
use crate::errors::Result;
use crate::strategies::ExplorationResult;

#[derive(Default)]
pub struct TextExtractor;

impl Analyzer for TextExtractor {
    fn analyze(&self, input: &ExplorationResult) -> Result<AnalysisOutput> {
        let facts = input
            .content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        Ok(AnalysisOutput { facts })
    }
}
