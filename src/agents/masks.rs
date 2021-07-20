use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, ChunksExactMut};

/// A convenience struct to encapsulate masks for each agent while maintaining contiguous memory
pub struct Masks {
    _v: Vec<bool>,
    // TODO investigate bitvec
    window_length: usize,
}

/// SoA layout of bitmasks showing the presence of an agent within the world
impl Masks {
    pub fn new(world_width: u32, world_height: u32, num_agents: u32) -> Self {
        let window_length = world_width as usize * world_height as usize;
        return Self {
            _v: vec![false; window_length * num_agents as usize],
            window_length,
        };
    }
}

/// Return the nth mask
impl Index<usize> for Masks {
    type Output = [bool];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        return &self._v[(self.window_length * index as usize)..((self.window_length * (index + 1)) + 1)];
    }
}

impl IndexMut<usize> for Masks {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self._v[(self.window_length * index as usize)..((self.window_length * (index + 1)) + 1)];
    }
}


impl<'a> IntoIterator for Masks {
    type Item = &'a [bool];
    type IntoIter = ChunksExact<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self._v.chunks_exact(self.window_length)
    }
}

impl<'a> IntoIterator for &'a Masks {
    type Item = &'a [bool];
    type IntoIter = ChunksExact<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self._v.chunks_exact(self.window_length)
    }
}

impl<'a> IntoIterator for &'a mut Masks {
    type Item = &'a mut [bool];
    type IntoIter = ChunksExactMut<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self._v.chunks_exact_mut(self.window_length)
    }
}
