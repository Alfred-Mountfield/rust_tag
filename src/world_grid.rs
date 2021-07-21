use std::ops::{Index, IndexMut};

use nonmax::NonMaxU32;

use crate::agents::{Agents, Coord};

//pre-calculated u32 colours from RGB found by ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32)

// [255, 0, 0]
const TAGGED_COLOUR: u32 = 16711680;
// [0, 255, 0]
const NORMAL_COLOUR: u32 = 65280;

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

        let tagged_idx = agents.tagged.iter().position(|&val| val).unwrap();
        buffer[self.coord_to_idx(&agents.pos[tagged_idx])] = TAGGED_COLOUR;

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