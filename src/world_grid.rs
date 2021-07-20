use crate::agents::Agents;

#[derive(Debug)]
pub struct WorldGrid {
    agents: Agents,
    width: u32,
    height: u32,
}

impl WorldGrid {
    pub fn new(width: u32, height: u32, num_agents: u32) -> Self {
        Self {
            agents: Agents::new(num_agents, width, height),
            width,
            height,
        }
    }


    pub fn as_buffer(&self) -> Vec<u32> {
        let mut buffer = vec![0u32; (self.width * self.height) as usize];
        self.agents.world_idx.iter()
            .zip(self.agents.tagged.iter())
            .for_each(|(&idx, &tagged)| {
                let rgb = if tagged { [0, 0, 255] } else { [0, 255, 0] };
                buffer[idx] = ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32;
            });

        buffer
    }
}