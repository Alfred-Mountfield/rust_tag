use nonmax::NonMaxU32;
use rand::{Rng, seq, thread_rng};

use crate::world_grid::WorldGrid;

const TAG_RADIUS: u32 = 2;
const AGENT_VIEW_DISTANCE: u32 = 8;

const MAX_VELOCITY: i32 = 1;

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
    pub has_tagged_in_sight: Vec<bool>,
    pub last_tagged: u32,
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
            has_tagged_in_sight: vec![false; num_agents as usize],
            last_tagged: 0,
        }
    }

    #[inline]
    pub fn update(&mut self, world_grid: &mut WorldGrid) {
        self.walk(world_grid);

        self.has_tagged_in_sight = vec![false; self.pos.len() as usize];

        let tagged_idx = self.tagged.iter().position(|&e| e).unwrap();
        let tagged_pos = self.pos[tagged_idx];

        // Keep track of the closest agent's position to the tagged agent
        let mut closest_dist = i64::MAX;
        let mut closest_pos = None;

        // Check a box around the player
        for y in (tagged_pos.y.saturating_sub(AGENT_VIEW_DISTANCE))..(tagged_pos.y.saturating_add(AGENT_VIEW_DISTANCE + 1)) {
            for x in (tagged_pos.x.saturating_sub(AGENT_VIEW_DISTANCE))..(tagged_pos.x.saturating_add(AGENT_VIEW_DISTANCE + 1)) {
                let dx = tagged_pos.x as i64 - x as i64;
                let dy = tagged_pos.y as i64 - y as i64;
                let sq_dist = (dx * dx) + (dy * dy);

                // Ensure indices are within the world, and prune non-circular ones
                if x < world_grid.width && y < world_grid.height && sq_dist < (AGENT_VIEW_DISTANCE * AGENT_VIEW_DISTANCE) as i64 {
                    // Check if there's an agent within the position
                    let nearby_coord = { Coord { x, y } };
                    if let Some(agent_idx) = world_grid[nearby_coord] {
                        let nearby_idx = agent_idx.get() as usize;

                        if nearby_idx != tagged_idx {
                            if sq_dist < closest_dist {
                                closest_dist = sq_dist;
                                closest_pos = Some(nearby_coord);
                            }

                            self.has_tagged_in_sight[nearby_idx] = true;
                            let mut run_away_vec = calc_vector(tagged_pos, nearby_coord);
                            run_away_vec.clamp(-1 * MAX_VELOCITY, MAX_VELOCITY);
                            self.vel[nearby_idx] = run_away_vec;
                        }
                    }
                }
            }
        }

        if let Some(pos) = closest_pos {
            let mut chase_vec = calc_vector(pos, tagged_pos);
            chase_vec.clamp(-1 * MAX_VELOCITY, MAX_VELOCITY);
            self.vel[tagged_idx] = chase_vec;
        }

        // Check if any agents are within range of being tagged
        for y in (tagged_pos.y.saturating_sub(TAG_RADIUS))..(tagged_pos.y.saturating_add(TAG_RADIUS + 1)) {
            for x in (tagged_pos.x.saturating_sub(TAG_RADIUS))..(tagged_pos.x.saturating_add(TAG_RADIUS + 1)) {
                if x < world_grid.width && y < world_grid.height {
                    if let Some(agent_idx) = world_grid[{ Coord { x, y } }] {
                        // Check it's not the tagged agent, or the last tagged
                        if agent_idx.get() as usize != tagged_idx && agent_idx.get() != self.last_tagged {
                            self.tagged[tagged_idx] = false;
                            self.tagged[agent_idx.get() as usize] = true;
                            self.last_tagged = tagged_idx as u32;
                            // println!("Tagged");
                            return;
                        }
                    }
                }
            }
        }
    }

    #[inline]
    /// Agents will meander in a general direction, rarely turning slightly and turning around when
    /// at a boundary
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
        y: pos_2.y as i32 - pos_1.y as i32,
    }
}