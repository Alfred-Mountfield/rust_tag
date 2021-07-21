use std::ops::{Index, IndexMut};

use nonmax::NonMaxU32;

use crate::agents::{Agents, Coord};

//pre-calculated u32 colours from RGB found by ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32

// [255, 0, 0]
const TAGGED_COLOUR: u32 = 16711680;
// [0, 255, 0]
const NORMAL_COLOUR: u32 = 65280;

/// How many more pixels to draw around the tagged player to find it easier visually
const TAGGED_SCALE_INCREASE: u32 = 0;

#[derive(Debug, Default)]
struct Cell {
    elements: Vec<usize>,
}

#[derive(Debug)]
pub struct WorldGrid {
    pub width: u32,
    pub height: u32,
    // NonMaxU32 used for efficient bytes-alignment with an Option for better cache performance
    pub agents: Vec<Option<NonMaxU32>>,
}

impl WorldGrid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            agents: vec![None; (width * height) as usize],
        }
    }

    #[inline]
    pub fn coord_to_idx(&self, coord: &Coord) -> usize {
        coord.x as usize + (coord.y as usize * self.width as usize)
    }

    #[inline]
    pub fn as_buffer(&self, agents: &Agents) -> Vec<u32> {
        let mut buffer = vec![0u32; (self.width * self.height) as usize];
        self.agents.iter().zip(buffer.iter_mut()).for_each(|(&agent_present, buffer_pixel)| {
            if agent_present.is_some() {
                *buffer_pixel = NORMAL_COLOUR;
            }
        });

        // Draw a slightly bigger box around the tagged player for purely visual purposes
        let tagged_idx = agents.tagged.iter().position(|&val| val).unwrap();
        let tagged_pos = &agents.pos[tagged_idx];
        for y_coord in (tagged_pos.y.saturating_sub(TAGGED_SCALE_INCREASE))..(tagged_pos.y.saturating_add(TAGGED_SCALE_INCREASE + 1)) {
            for x_coord in (tagged_pos.x.saturating_sub(TAGGED_SCALE_INCREASE))..(tagged_pos.x.saturating_add(TAGGED_SCALE_INCREASE + 1)) {
                if y_coord < self.height && x_coord < self.width {
                    buffer[self.coord_to_idx(&Coord{x: x_coord, y: y_coord})] = TAGGED_COLOUR;
                }
            }
        }

        // Draw a slightly bigger box around the tagged player for purely visual purposes
        agents.has_tagged_in_sight.iter().enumerate().filter(|(_, &within_radius)| within_radius).for_each(|(idx, _)| {
            buffer[self.coord_to_idx(&agents.pos[idx])] = 255;
        });

        buffer
    }
}

impl Index<Coord> for WorldGrid {
    type Output = Option<NonMaxU32>;

    #[inline]
    fn index(&self, coord: Coord) -> &Self::Output {
        &self.agents[self.coord_to_idx(&coord)]
    }
}

impl IndexMut<Coord> for WorldGrid {
    #[inline]
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        let idx = self.coord_to_idx(&coord);
        &mut self.agents[idx]
    }
}