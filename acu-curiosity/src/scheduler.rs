use std::collections::VecDeque;

use crate::{errors::Result, ExplorationId, ExplorationPlan};

#[derive(Default)]
pub struct Scheduler {
    queue: VecDeque<(ExplorationId, ExplorationPlan)>,
}

impl Scheduler {
    pub fn schedule(&mut self, id: ExplorationId, plan: ExplorationPlan) {
        self.queue.push_back((id, plan));
    }

    pub fn next_plan(&mut self) -> Result<Option<(ExplorationId, ExplorationPlan)>> {
        Ok(self.queue.pop_front())
    }
}
