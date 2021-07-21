use minifb::{Key, Scale, Window, WindowOptions};

use rust_tag::agents::Agents;
use rust_tag::world_grid::WorldGrid;

const WORLD_WIDTH: u32 = 600;
const WORLD_HEIGHT: u32 = 400;
const NUM_AGENTS: u32 = 10_000;

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
            // borderless: true,
            ..WindowOptions::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    window.set_position(0, 0);


    // ~60fps
    // 16600
    // ~7.5fps
    // 132800
    // ~3.5fps
    let mut limit_update_rate = 265600;

    window.limit_update_rate(Some(std::time::Duration::from_micros(limit_update_rate)));

    let mut moving_avg = 0f64;
    let alpha = 1.0/20.0;

    let mut counter = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            limit_update_rate = (limit_update_rate as f64 * 1.5) as u64;
            window.limit_update_rate(Some(std::time::Duration::from_micros(limit_update_rate)));
        } else if window.is_key_down(Key::Right) {
            limit_update_rate = (limit_update_rate as f64 * 0.8) as u64;
            window.limit_update_rate(Some(std::time::Duration::from_micros(limit_update_rate)));
        }

        let frame_timer = std::time::Instant::now();
        agents.update(&mut world);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&world.as_buffer(&agents), WORLD_WIDTH as usize, WORLD_HEIGHT as usize)
            .unwrap();

        let elapsed = frame_timer.elapsed();
        // exponential moving-average
        moving_avg = (alpha * elapsed.as_micros() as f64) + (1.0 - alpha) * moving_avg;

        counter += 1;
        if counter % 60 == 0 {
            counter = 0;
            let fps = 1_000_000.0 / moving_avg;
            dbg!(fps);
        }

    }
}
