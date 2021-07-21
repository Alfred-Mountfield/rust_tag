use nonmax::NonMaxU32;
use rand::{Rng, seq, thread_rng};

use crate::world_grid::WorldGrid;

const TAG_RADIUS: u32 = 5;

/// A 2-dimensional vector for non-negative integers
#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

/// A 2-dimensional vector for integers
#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    #[inline]
    fn clamp(&mut self, min: i32, max: i32) {
        if self.x > max {
            self.x = max;
        } else if self.x < min {
            self.x = min;
        }
        if self.y > max {
            self.y = max;
        } else if self.y < min {
            self.y = min;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Agents {
    pub tagged: Vec<bool>,
    pub pos: Vec<Coord>,
    pub vel: Vec<Vec2>,
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

        let vel = (0..num_agents).map(|_| {
            let x_mag = rng.gen_range(-3..3);
            let y_mag = rng.gen_range(-3..3);
            Vec2 { x: x_mag, y: y_mag }
        }).collect();

        Self {
            tagged,
            pos,
            vel,
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
        for (idx, (pos, vel)) in self.pos.iter_mut().zip(self.vel.iter_mut()).enumerate() {
            if rng.gen::<f32>() < 0.1 {
                let random: f32 = rng.gen();
                if random < 0.5 {
                    if random < 0.25 { vel.x -= 1; } else { vel.x += 1; }
                } else {
                    if random < 0.75 { vel.y -= 1; } else { vel.y += 1; }
                }
                vel.clamp(-2, 2);
            }
            let mut new_x = pos.x as i32 + vel.x;
            let mut new_y = pos.y as i32 + vel.y;

            if new_x >= world_grid.width as i32 {
                new_x = (world_grid.width - 1) as i32;
                vel.x *= -1;
            };
            if new_x < 0 {
                new_x = 0;
                vel.x *= -1;
            };

            if new_y >= world_grid.height as i32 {
                new_y = (world_grid.height - 1) as i32;
                vel.y *= -1;
            };
            if new_y < 0 {
                new_y = 0;
                vel.y *= -1;
            };

            if world_grid[{ Coord { x: new_x as u32, y: new_y as u32 } }].is_none() {
                world_grid[*pos] = None;

                pos.x = new_x as u32;
                pos.y = new_y as u32;

                world_grid[*pos] = NonMaxU32::new(idx as u32);
            }
        }
    }
}
