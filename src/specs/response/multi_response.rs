use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use fake::rand::{self, seq::IndexedRandom};
use serde::{Deserialize, Serialize};

use crate::specs::response::{Response, Strategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiResponse {
    #[serde(default)]
    pub strategy: Strategy,
    pub responses: Vec<Response>,

    #[serde(skip)]
    pub cycle_id: Arc<AtomicUsize>,
}

impl MultiResponse {
    /// Gets teh endpoint response based on the set strategy.
    pub fn get(&self) -> &Response {
        if self.responses.is_empty() {
            panic!("MultiResponse contains no responses!");
        }

        match self.strategy {
            Strategy::Random => {
                let mut rng = rand::rng();
                // Safe to unwrap since we check if responses are not empty
                self.responses.choose(&mut rng).unwrap()
            }
            Strategy::Cycle => {
                let current = self.cycle_id.fetch_add(1, Ordering::SeqCst);
                let id = current % self.responses.len();
                &self.responses[id]
            }
        }
    }
}
