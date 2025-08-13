use crate::errors::Result;
use crate::strategies::ExplorationResult;

#[derive(Default)]
pub struct Embedder;

impl Embedder {
    pub fn embed(&self, _input: &ExplorationResult) -> Result<Vec<f32>> {
        Ok(vec![0.0; 4])
    }
}
