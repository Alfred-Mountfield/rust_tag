use rand::{seq, thread_rng};

#[derive(Debug, Clone)]
pub struct Agents {
    pub tagged: Vec<bool>,
    pub world_idx: Vec<usize>,
}

impl Agents {
    pub fn new(num_agents: u32, world_width: u32, world_height: u32) -> Self {
        let mut rng = thread_rng();
        let world_idx = seq::index::sample(&mut rng, (world_width * world_height) as usize, num_agents as usize).into_vec();
        let mut tagged = vec![false; num_agents as usize];
        // Select the first agent as the tagged player
        tagged[0] = true;

        Self {
            tagged,
            world_idx,
        }
    }
}
