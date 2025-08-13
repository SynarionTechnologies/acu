use crate::errors::Result;
use crate::strategies::ExplorationResult;

pub trait Analyzer {
    fn analyze(&self, input: &ExplorationResult) -> Result<AnalysisOutput>;
}

#[derive(Debug, Clone)]
pub struct AnalysisOutput {
    pub facts: Vec<String>,
}

pub mod embedding;
pub mod text_extract;
