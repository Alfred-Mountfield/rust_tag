use minifb::{Key, Scale, Window, WindowOptions};

use crate::world_grid::WorldGrid;

mod agents;
mod world_grid;

const WORLD_WIDTH: u32 = 40;
const WORLD_HEIGHT: u32 = 40;
const NUM_AGENTS: u32 = 10;

fn main() {
    let world = WorldGrid::new(WORLD_WIDTH, WORLD_HEIGHT, NUM_AGENTS);

    let mut window = Window::new(
        "Tag",
        WORLD_WIDTH as usize,
        WORLD_HEIGHT as usize,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            ..WindowOptions::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&world.as_buffer(), WORLD_WIDTH as usize, WORLD_HEIGHT as usize)
            .unwrap();
    }
}
