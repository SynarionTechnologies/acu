use thiserror::Error;

pub type Result<T> = std::result::Result<T, CuriosityError>;

#[derive(Debug, Error)]
pub enum CuriosityError {
    #[error("strategy not found: {0}")]
    StrategyNotFound(String),
    #[error("scheduler error: {0}")]
    Scheduler(String),
    #[error("execution error: {0}")]
    Execution(String),
}
