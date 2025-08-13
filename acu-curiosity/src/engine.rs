use std::collections::HashMap;

use uuid::Uuid;

use crate::analyzers::text_extract::TextExtractor;
use crate::analyzers::{AnalysisOutput, Analyzer};
use crate::config::CuriosityConfig;
use crate::errors::{CuriosityError, Result};
use crate::scheduler::Scheduler;
use crate::strategies::api_query::ApiQueryStrategy;
use crate::strategies::corpus_search::CorpusSearchStrategy;
use crate::strategies::web_crawl::WebCrawlStrategy;
use crate::strategies::{ExplorationResult, ExplorationStrategy};

pub type ExplorationId = Uuid;

#[derive(Clone)]
pub struct ExplorationPlan {
    pub strategy: String,
    pub target: String,
}

pub struct AnalysisReport {
    pub facts: Vec<String>,
}

pub enum CuriosityEvent {
    CuriosityExplorationStarted(ExplorationId),
    CuriosityExplorationCompleted(ExplorationId),
}

pub trait CuriosityEngine {
    fn start_exploration(&mut self, plan: ExplorationPlan) -> Result<ExplorationId>;
    fn tick(&mut self) -> Result<()>;
    fn analyze_results(&mut self, exploration_id: &ExplorationId) -> Result<AnalysisReport>;
}

pub struct DefaultCuriosityEngine {
    scheduler: Scheduler,
    strategies: HashMap<String, Box<dyn ExplorationStrategy + Send + Sync>>,
    analyzer: TextExtractor,
    logs: HashMap<ExplorationId, ExplorationResult>,
    events: Vec<CuriosityEvent>,
    _config: CuriosityConfig,
}

impl DefaultCuriosityEngine {
    pub fn new(config: CuriosityConfig) -> Self {
        let mut strategies: HashMap<String, Box<dyn ExplorationStrategy + Send + Sync>> =
            HashMap::new();
        strategies.insert("web_crawl".to_string(), Box::new(WebCrawlStrategy));
        strategies.insert("api_query".to_string(), Box::new(ApiQueryStrategy));
        strategies.insert("corpus_search".to_string(), Box::new(CorpusSearchStrategy));
        Self {
            scheduler: Scheduler::default(),
            strategies,
            analyzer: TextExtractor,
            logs: HashMap::new(),
            events: Vec::new(),
            _config: config,
        }
    }

    pub fn events(&self) -> &[CuriosityEvent] {
        &self.events
    }
}

impl CuriosityEngine for DefaultCuriosityEngine {
    fn start_exploration(&mut self, plan: ExplorationPlan) -> Result<ExplorationId> {
        let id = ExplorationId::new_v4();
        self.scheduler.schedule(id, plan);
        self.events
            .push(CuriosityEvent::CuriosityExplorationStarted(id));
        Ok(id)
    }

    fn tick(&mut self) -> Result<()> {
        if let Some((id, plan)) = self.scheduler.next_plan()? {
            let strategy_name = plan.strategy.clone();
            let strategy = self
                .strategies
                .get(&strategy_name)
                .ok_or(CuriosityError::StrategyNotFound(strategy_name))?;
            let result = strategy.execute(&plan)?;
            self.logs.insert(id, result);
            self.events
                .push(CuriosityEvent::CuriosityExplorationCompleted(id));
        }
        Ok(())
    }

    fn analyze_results(&mut self, exploration_id: &ExplorationId) -> Result<AnalysisReport> {
        let result = self
            .logs
            .get(exploration_id)
            .ok_or_else(|| CuriosityError::Execution("missing log".into()))?;
        let AnalysisOutput { facts } = self.analyzer.analyze(result)?;
        Ok(AnalysisReport { facts })
    }
}
