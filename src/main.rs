use minifb::{Key, Scale, Window, WindowOptions};

use rust_tag::agents::Agents;
use rust_tag::world_grid::WorldGrid;

const WORLD_WIDTH: u32 = 1900;
const WORLD_HEIGHT: u32 = 1500;
const NUM_AGENTS: u32 = 20_000;

fn main() {
    let mut world = WorldGrid::new(WORLD_WIDTH, WORLD_HEIGHT);
    let mut agents = Agents::new(NUM_AGENTS, &mut world);

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
    // Limit to max ~7.5 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(132800)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        agents.update(&mut world);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&world.as_buffer(&agents), WORLD_WIDTH as usize, WORLD_HEIGHT as usize)
            .unwrap();
    }
}
