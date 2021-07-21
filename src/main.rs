use minifb::{Key, Scale, Window, WindowOptions};

use rust_tag::agents::Agents;
use rust_tag::world_grid::WorldGrid;

const WORLD_WIDTH: u32 = 200;
const WORLD_HEIGHT: u32 = 200;
const NUM_AGENTS: u32 = 5_000;

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
    window.set_position(0, 0);

    /// Initial limit on how quickly a frame is allowed to update. ~60fps: 16600, ~7.5fps: 132800, ~3.5fps: 265600
    let mut limit_update_rate = 265600;
    window.limit_update_rate(Some(std::time::Duration::from_micros(limit_update_rate)));

    let mut moving_avg = 0f64;
    let moving_avg_alpha = 1.0/20.0;

    let mut tagged_counter = 0;
    let mut total_frames: i128 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_timer = std::time::Instant::now();

        if window.is_key_down(Key::Left) {
            limit_update_rate = (limit_update_rate as f64 * 2.0) as u64;
            window.limit_update_rate(Some(std::time::Duration::from_micros(limit_update_rate)));
        } else if window.is_key_down(Key::Right) {
            limit_update_rate = (limit_update_rate as f64 * 0.8) as u64;
            window.limit_update_rate(Some(std::time::Duration::from_micros(limit_update_rate)));
        }

        let tagged_this_frame = agents.update(&mut world);
        if tagged_this_frame { tagged_counter += 1 };

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&world.as_buffer(&agents), WORLD_WIDTH as usize, WORLD_HEIGHT as usize)
            .unwrap();

        let elapsed = frame_timer.elapsed();
        // exponential moving-average
        moving_avg = (moving_avg_alpha * elapsed.as_micros() as f64) + (1.0 - moving_avg_alpha) * moving_avg;

        total_frames += 1;
    }

    println!("Ran for {} time-steps", total_frames);
    println!("Averaged {:.2} time-steps between each tag", total_frames as f64 / tagged_counter as f64);
    println!("Average frame-rate: {:.2}fps", 1_000_000.0 / moving_avg);
}
