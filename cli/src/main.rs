use clap::Parser;

use mpi::{traits::*};

use insploray_cli::{CliConfig, load_scene, mpi::{master::run_master, worker::run_worker}, non_cluster_render};

fn main() {
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let cli_config = CliConfig::parse();

    if size == 1 {
        let (scene, cam) = load_scene(&cli_config);
        non_cluster_render(&cli_config, cores, scene, cam);
    }
    else {
        if rank == 0 {
            run_master(&world, &cli_config);
        }
        else {
            run_worker(&world, rank, cores as u32);
        }
    }
}
