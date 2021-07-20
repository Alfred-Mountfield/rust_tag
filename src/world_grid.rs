use crate::agent::Agent;
use rand::{seq, thread_rng};

#[derive(Debug)]
pub struct WorldGrid {
    cells: Vec<Option<Agent>>,
}

impl WorldGrid {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let mut rng = thread_rng();
        let agent_indices = seq::index::sample(&mut rng, width * height, num_agents);
        let mut cells = vec![None; width * height];
        agent_indices.iter().for_each(|idx| {
            cells[idx] = Some(Default::default())
        });

        Self {
            cells
        }
    }
}
