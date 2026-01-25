use insploray::accumulators::{Accumulator, TileAccumulator}; 
use insploray::file_formats::ExrImage;

use mpi::traits::{Communicator, Destination, Source};

use crate::{CliConfig, load_scene};
use crate::mpi::{TAG_JOB, TAG_RESOURCE_INFO, TAG_RESULT, TAG_SHUTDOWN, utils::broadcast_data};

pub fn run_master(world: &impl Communicator, cli_config: &CliConfig) {
    let cluster_size = world.size();
    println!("Cluster Size : {}", cluster_size);
    let mut cores : Vec<u32> = vec![0; cluster_size as usize];

    for _ in 0..(cluster_size - 1) {
        let (res, stat) = world.any_process().receive_with_tag::<u32>(TAG_RESOURCE_INFO);
        cores[stat.source_rank() as usize] = res;
    }
    println!("Cores : {:?}", cores);

    let process = world.this_process();

    let (scene, cam) = load_scene(&cli_config);

    broadcast_data(&process, 0, Some(&scene.bvh));
    broadcast_data(&process, 0, Some(&scene.materials));
    broadcast_data(&process, 0, Some(&cam));

    let sky_data = match scene.skybox {
        Some(ref skybox) => {
            let buff = skybox.get_buffer();
            (scene.default_sky_color, Some((buff, skybox.get_dimensions())))
        },
        None => {
            (scene.default_sky_color, None)
        }
    };
    broadcast_data(&process, 0, Some(&sky_data));
    drop(scene);
    drop(cam);

    // Job Scheduling
    let samples = cli_config.samples;
    let samples_per_core = samples / cores.iter().sum::<u32>();
    for i in 1..cores.len() {
        let msg = vec![cli_config.width, cli_config.height, samples_per_core * cores[i]];
        world.process_at_rank(i as i32).send_with_tag(&msg[..], TAG_JOB);
    }
    
    // Aggregating
    let mut accumulator = Accumulator::new(cli_config.width, cli_config.height);

    for _ in 1..cluster_size {
        let (node_res, status) = world.any_process().receive_vec_with_tag::<f32>(TAG_RESULT);
        let samples_calculated = samples_per_core * cores[status.source_rank() as usize];
        let mut tile_accumulator = TileAccumulator::new(0, 0, cli_config.width, cli_config.height);

        // let pixels: &[Vec4] = insploray::cast_slice(&node_res);
        tile_accumulator.framebuffer = insploray::cast_slice(&node_res).to_vec();
        tile_accumulator.sample_counts = vec![samples_calculated; (cli_config.width * cli_config.height) as usize];

        accumulator.merge_tile(tile_accumulator);
        println!("Collected work form rank {}", status.source_rank()); 
    }

    let buff = accumulator.get_image_buffer();

    let exr = ExrImage {
        rgb: buff
    };

    exr.save_to_files(&cli_config.output);

    shutdown_cluster(world);
}

fn shutdown_cluster(world: &impl Communicator) {
    for i in 1..world.size() {
        world.process_at_rank(i).send_with_tag(&0, TAG_SHUTDOWN);
    }
}
