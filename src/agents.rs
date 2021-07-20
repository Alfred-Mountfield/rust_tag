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

        let pos = seq::index::sample(&mut rng, (world_grid.width * world_grid.height) as usize, num_agents as usize)
            .into_iter()
            .map(|idx| {
                let y_pos = idx as u32 / world_grid.width;
                let x_pos = idx as u32 - (y_pos * world_grid.width);
                Coord { x: x_pos, y: y_pos }
            })
            .collect();

        Self {
            tagged,
            pos,
        }
    }

    pub fn update(&mut self, world_grid: &mut WorldGrid) {
        self.walk(world_grid);

        // Slow and *very* basic manhattan distance calc to find agents nearby to the tagged agent
        let tag_idx = self.tagged.iter().position(|&e| e == true).unwrap();
        let tag_pos = self.pos[tag_idx];
        for (idx, other_pos) in self.pos.iter().enumerate() {
            if idx != tag_idx {
                if ((tag_pos.x as i64 - other_pos.x as i64).abs() as u32) < TAG_RADIUS
                    && ((tag_pos.y as i64 - other_pos.y as i64).abs() as u32) < TAG_RADIUS {
                    self.tagged[tag_idx] = false;
                    self.tagged[idx] = true;
                    break;
                }
            }
        }
    }

    fn walk(&mut self, world_grid: &mut WorldGrid) {
        let mut rng = thread_rng();
        for pos in self.pos.iter_mut() {
            let mut new_x = pos.x as i64 + (rng.gen_range(-1..2));
            let mut new_y = pos.y as i64 + (rng.gen_range(-1..2));

            if new_x >= world_grid.width as i64 { new_x = (world_grid.width - 1) as i64 };
            if new_x < 0 { new_x = 0 };

            if new_y >= world_grid.height as i64 { new_y = (world_grid.height - 1) as i64 };
            if new_y < 0 { new_y = 0 };

            pos.x = new_x as u32;
            pos.y = new_y as u32;
        }
    }
}
