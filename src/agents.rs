use nonmax::NonMaxU32;
use rand::{Rng, seq, thread_rng};

use crate::world_grid::WorldGrid;

const TAG_RADIUS: u32 = 4;

const MAX_VELOCITY: i32 = 2;

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
    pub within_vicinity_of_tagged: Vec<bool>,
    pub last_tagged: u32
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
            within_vicinity_of_tagged: vec![false; num_agents as usize],
            last_tagged: 0
        }
    }

    #[inline]
    pub fn update(&mut self, world_grid: &mut WorldGrid) {
        self.walk(world_grid);

        self.within_vicinity_of_tagged = vec![false; self.pos.len() as usize];

        let tag_idx = self.tagged.iter().position(|&e| e).unwrap();
        let tag_pos = self.pos[tag_idx];

        for y in (tag_pos.y.saturating_sub(TAG_RADIUS * 5))..(tag_pos.y.saturating_add(TAG_RADIUS * 5 + 1)) {
            for x in (tag_pos.x.saturating_sub(TAG_RADIUS * 5))..(tag_pos.x.saturating_add(TAG_RADIUS * 5 + 1)) {
                if x < world_grid.width && y < world_grid.height {
                    let nearby_coord = { Coord { x, y } };
                    if let Some(agent_idx) = world_grid[nearby_coord] {
                        let nearby_idx = agent_idx.get() as usize;

                        if nearby_idx != tag_idx {
                            self.within_vicinity_of_tagged[nearby_idx] = true;
                            self.vel[nearby_idx] = calc_vector(tag_pos, nearby_coord);
                        }
                    }
                }
            }
        }

        for y in (tag_pos.y.saturating_sub(TAG_RADIUS))..(tag_pos.y.saturating_add(TAG_RADIUS + 1)) {
            for x in (tag_pos.x.saturating_sub(TAG_RADIUS))..(tag_pos.x.saturating_add(TAG_RADIUS + 1)) {
                if x < world_grid.width && y < world_grid.height {
                    if let Some(agent_idx) = world_grid[{ Coord { x, y } }] {
                        // Check it's not the tagged agent, or the last tagged
                        if agent_idx.get() as usize != tag_idx  && agent_idx.get() != self.last_tagged {
                            self.tagged[tag_idx] = false;
                            self.tagged[agent_idx.get() as usize] = true;
                            self.last_tagged = tag_idx as u32;
                            println!("Tagged");
                            return;
                        }
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
                vel.clamp(-1 * MAX_VELOCITY, MAX_VELOCITY);
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

fn calc_vector(pos_1: Coord, pos_2: Coord) -> Vec2 {
    Vec2 {
        x: pos_2.x as i32 - pos_1.x as i32,
        y: pos_2.y as i32 - pos_1.y as i32
    }
}