use rand::{seq, thread_rng};

use crate::agent::Agent;

#[derive(Debug)]
pub struct WorldGrid {
    cells: Vec<Option<Agent>>,
    width: usize,
    height: usize,
}

impl WorldGrid {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let mut rng = thread_rng();
        let agent_indices = seq::index::sample(&mut rng, width * height, num_agents);
        let mut cells: Vec<Option<Agent>> = vec![None; width * height];
        agent_indices.iter().for_each(|idx| {
            cells[idx] = Some(Default::default())
        });

        // Select the first agent as the tagged player
        cells[agent_indices.index(0)].as_mut().unwrap().tagged = true;

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn as_buffer(&self) -> Vec<u32> {
        self.cells.iter().map(|is_agent| {
            // return an RGB chunk showing if there's an agent or not
            let rgb = is_agent.as_ref().map_or_else(
                || [0, 0, 0], // no agent
                |agent| {
                    if agent.tagged { [0, 0, 255] } else { [0, 255, 0] }
                },
            );
            // convert the RGB array into a u32
            ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32
        }).collect()
    }
}