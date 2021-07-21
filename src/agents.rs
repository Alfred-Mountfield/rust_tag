use nonmax::NonMaxU32;
use rand::{Rng, seq, thread_rng};

use crate::world_grid::WorldGrid;

const TAG_RADIUS: u32 = 3;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub struct Agents {
    pub tagged: Vec<bool>,
    pub pos: Vec<Coord>,
}

impl Agents {
    pub fn new(num_agents: u32, world_grid: &mut WorldGrid) -> Self {
        let mut rng = thread_rng();

        let mut tagged = vec![false; num_agents as usize];
        // Select the first agent as the tagged player
        tagged[0] = true;

        let pos: Vec<Coord> = seq::index::sample(&mut rng, (world_grid.width * world_grid.height) as usize, num_agents as usize)
            .into_iter()
            .map(|idx| {
                let y_pos = idx as u32 / world_grid.width;
                let x_pos = idx as u32 - (y_pos * world_grid.width);
                Coord { x: x_pos, y: y_pos }
            })
            .collect();

        // Slightly inefficient but split for clarity
        pos.iter().enumerate().for_each(|(agent_idx, &coord)| {
            world_grid[coord] = NonMaxU32::new(agent_idx as u32);
        });

        Self {
            tagged,
            pos,
        }
    }

    #[inline]
    pub fn update(&mut self, world_grid: &mut WorldGrid) {
        self.walk(world_grid);
        // Slow and *very* basic manhattan distance calc to find agents nearby to the tagged agent
        let tag_idx = self.tagged.iter().position(|&e| e).unwrap();
        let tag_pos = self.pos[tag_idx];

        for y in (tag_pos.y.saturating_sub(TAG_RADIUS))..(tag_pos.y + TAG_RADIUS) {
            for x in (tag_pos.x.saturating_sub(TAG_RADIUS))..(tag_pos.x + TAG_RADIUS) {
                if x < world_grid.width && y < world_grid.height {
                    if let Some(agent_idx) = world_grid[{ Coord { x, y } }] {
                        self.tagged[tag_idx] = false;
                        self.tagged[agent_idx.get() as usize] = true;
                        break;
                    }
                }
            }
        }
    }

    #[inline]
    fn walk(&mut self, world_grid: &mut WorldGrid) {
        let mut rng = thread_rng();
        for (idx, pos) in self.pos.iter_mut().enumerate() {
            let mut new_x = pos.x as i64 + (rng.gen_range(-1..2));
            let mut new_y = pos.y as i64 + (rng.gen_range(-1..2));

            if new_x >= world_grid.width as i64 { new_x = (world_grid.width - 1) as i64 };
            if new_x < 0 { new_x = 0 };

            if new_y >= world_grid.height as i64 { new_y = (world_grid.height - 1) as i64 };
            if new_y < 0 { new_y = 0 };

            if world_grid[{ Coord { x: new_x as u32, y: new_y as u32 } }].is_none() {
                world_grid[*pos] = None;

                pos.x = new_x as u32;
                pos.y = new_y as u32;

                world_grid[*pos] = NonMaxU32::new(idx as u32);
            }
        }
    }
}
