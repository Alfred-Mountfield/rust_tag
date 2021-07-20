use crate::agents::{Agents, Coord};

#[derive(Debug, Default)]
struct Cell {
    elements: Vec<usize>
}

#[derive(Debug)]
pub struct WorldGrid {
    pub width: u32,
    pub height: u32,
}

impl WorldGrid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    #[inline]
    pub fn coord_to_idx(&self, coord: &Coord) -> usize {
        coord.x as usize + (coord.y as usize * self.width as usize)
    }

    #[inline]
    pub fn as_buffer(&self, agents: &Agents) -> Vec<u32> {
        let mut buffer = vec![0u32; (self.width * self.height) as usize];
        agents.pos.iter()
            .zip(agents.tagged.iter())
            .for_each(|(pos, &tagged)| {
                let rgb = if tagged { [0, 0, 255] } else { [0, 255, 0] };
                let idx = self.coord_to_idx(pos);
                buffer[idx] = ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32;
            });

        buffer
    }
}