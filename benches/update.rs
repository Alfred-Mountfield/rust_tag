use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use rust_tag::world_grid::WorldGrid;
use rust_tag::agents::Agents;

fn setup(world_width: u32, world_height: u32, num_agents: u32) -> Box<dyn FnMut() -> (WorldGrid, Agents)> {
    Box::new(move || {
        let mut world = WorldGrid::new(world_width, world_height);
        let agents = Agents::new(num_agents, &mut world);
        (world, agents)
    })
}

fn run_update((mut world, mut agents): (WorldGrid, Agents)) {
    agents.update(&mut world);
}

fn bench_update_density_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Density - 100 x 100");

    let world_width = 100;
    let world_height = 100;

    for num_agents in [10, 20, 40, 80] {
        group.bench_with_input(BenchmarkId::from_parameter(format!("{}", num_agents)), &num_agents, |b, _| {
            b.iter_batched(
                setup(world_width, world_height, num_agents),
                run_update,
                BatchSize::SmallInput,
            )
        });
    }
}

fn bench_update_density_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("Density - 1_000 x 1_000");

    let world_width = 1000;
    let world_height = 1000;

    for num_agents in [10_000, 50_000, 100_000, 20_000] {
        group.bench_with_input(BenchmarkId::from_parameter(format!("{}", num_agents)), &num_agents, |b, _| {
            b.iter_batched(
                setup(world_width, world_height, num_agents),
                run_update,
                BatchSize::LargeInput,
            )
        });
    }
}

fn bench_update_density_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Density - 10_000 x 10_000");

    let world_width = 10_000;
    let world_height = 10_000;

    for num_agents in [50_000, 100_000, 500_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(format!("{}", num_agents)), &num_agents, |b, _| {
            b.iter_batched(
                setup(world_width, world_height, num_agents),
                run_update,
                BatchSize::LargeInput,
            )
        });
    }
}

criterion_group!(benches, bench_update_density_small, bench_update_density_medium, bench_update_density_large);
criterion_main!(benches);