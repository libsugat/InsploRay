use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;

use insploray::renderer::RayTracer;

use insploray::Vec3;
use insploray::cameras::{PinholeCamera};
use insploray::scene::{Scene, obj_loader};

pub mod mpi;

/// InsploRay
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliConfig {

    /// Path to scene to render, ends with .obj
    #[arg(name = "file")]
    pub input_file_path: Option<String>,
    /// Path to output file. Ending with .exr
    #[arg(short, long)]
    pub output: String,

    /// Samples per pixel
    #[arg(short, long, default_value_t = 64)]
    pub samples: u32,
    /// Seed value for sampler
    #[arg(long)]
    pub seed: Option<u32>,
    /// image width
    #[arg(short = 'W', long, default_value_t = 1280)]
    pub width: u32,
    /// image height
    #[arg(short = 'H', long, default_value_t = 720)]
    pub height: u32,
    /// Tile size
    #[arg(short, long, default_value_t = 32)]
    pub tile: u32,
    /// Maximum number of bounces
    #[arg(short, long, default_value_t = 5)]
    pub bounces: u32,

    /// No of threads to be used by the program to render image (no of cpu cores is recomended)
    #[arg(long)]
    pub nthreads: Option<usize>,
}

pub fn load_scene(cli_config: &CliConfig) -> (Scene, PinholeCamera) {
    let position = Vec3::new(9.5, 2.25, 0.0);
    let cam = PinholeCamera::new(
        position,
        Vec3::new(0.0, PI / 2.0, 0.0),
        50.0,
        36.0,
        [cli_config.width, cli_config.height],
    );

    let mut scene = Scene {
        spheres: vec![],
        materials: vec![],
        default_sky_color: 0.031 * Vec3::ONE,

        ..Default::default()
    };

    match &cli_config.input_file_path {
        Some(file) => {
            println!("Trying loading {}", file);
            match obj_loader::load_from_file(&file) {
                Ok((meshes, materials)) => {
                    scene.meshes = Some(meshes);
                    scene.materials = materials;
                }
                Err(e) => {
                    println!("Error loading scene : {}", e);
                }
            }
        },
        None => {
            scene = Scene::get_example_scene();
        }
    }

    println!("Building BVH");
    scene.build_bvh();
    println!("Building BVH Done");

    (scene, cam)
}

pub fn non_cluster_render(cli_config: &CliConfig, cores : usize, scene: Scene, cam: PinholeCamera) {
    println!("Running in Non Cluster mode");
    let arc_scene = Arc::new(scene);

    let mut renderer = RayTracer::new(cli_config.width, cli_config.height);
    renderer.set_tp_size(match cli_config.nthreads {
        None => cores,
        Some(n) => {
            if n < cores {
                n
            } else {
                 cores
            }
        }
    });

    renderer.set_active_camera(Arc::new(cam));
    renderer.update(cli_config.width, cli_config.height);
    println!("updated renderer and camera");

    let start_instance = Instant::now();

    for i in 0..cli_config.samples {
        print!("Sample no: {}; ", i+1);
        use std::io::{Write};
        std::io::stdout().flush().unwrap();
        renderer.render(&arc_scene);
        println!("{:?}", renderer.get_last_render_time());
        print!("{:3}% Done \t", (i * 100)/cli_config.samples);
    }

    let time_elapsed = start_instance.elapsed();
    renderer.save_exr(&cli_config.output);

    println!("Rendering took took {time_elapsed:?}");
}
