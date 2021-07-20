use crate::world_grid::WorldGrid;

mod agent;
mod world_grid;

const WORLD_WIDTH: usize = 5;
const WORLD_HEIGHT: usize = 5;
const NUM_AGENTS: usize = 5;

fn main() {
    let world = WorldGrid::new(WORLD_WIDTH, WORLD_HEIGHT, NUM_AGENTS);

    println!("{:?}", world);
}
