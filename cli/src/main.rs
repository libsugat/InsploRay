use std::f32::consts::PI;
use std::fmt::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc};
use std::time::Instant;

use clap::Parser;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use insploray::renderer::RayTracer;
use insploray::Vec3;
use insploray::cameras::PinholeCamera;
use insploray::scene::{Scene, obj_loader};

use insploray_cli::{CliConfig, get_cpu_count};

fn main() {
    // load_scene();
    // build_camera();
    // create_renderer();
    // create_scheduler();
    // render();
    // write_exr();

    // Parse arguments form the command line;
    let cli_config = CliConfig::parse();

    // Setup Camera
    let position = Vec3::new(9.5, 2.25, 0.0);
    let cam = PinholeCamera::new(
        position,
        Vec3::new(0.0, PI / 2.0, 0.0),
        // Vec3::new(0.0, PI / 2.0 , PI),
        // Vec3::ZERO,
        50.0,
        36.0,
        [cli_config.width, cli_config.height],
    );

    // Load Scene
    let mut scene = Scene {
        spheres: vec![],
        materials: vec![],
        default_sky_color: 0.031 * Vec3::ONE,

        ..Default::default()
    };

    match &cli_config.input_file_path {
        Some(file) => {
            println!("Loading {}", file);
            match obj_loader::load_from_file(&file) {
                Ok((meshes, tris, materials)) => {
                    scene.meshes = meshes;
                    scene.tris_vec = Some(tris);
                    scene.materials = materials;
                    println!("Scene loaded successfully..");
                }
                Err(e) => {
                    println!("Error loading scene : {}", e);
                    return;
                }
            }
        },
        None => {
            scene = insploray::scene::get_example_scene();
        }
    }

    // Build Acceleration Structure
    println!("Building BVH");
    scene.build_bvh();
    println!("Building BVH Done");

    let arc_scene = Arc::new(scene);

    // Get Core affininty and jobs from ci
    let cpu_count = get_cpu_count();
    let workers = if cli_config.threads == 0 { 
        cpu_count
    } else {
        cli_config.threads
    };
    println!("Using {} workers", workers);

    let mut renderer = RayTracer::new(cli_config.width, cli_config.height);
    renderer.set_tile_size(cli_config.tile);
    renderer.set_tp_size(workers);
    renderer.set_active_camera(Arc::new(cam));
    renderer.update(cli_config.width, cli_config.height);
    

    let start_instant = Instant::now();
    let last_render = Arc::new(AtomicU64::new(0.0_f64.to_bits()));

    // Setup Progress Bar
    let pb = ProgressBar::new(cli_config.samples as u64);
    let last_render_clone = Arc::clone(&last_render);
    pb.set_style(
        ProgressStyle::with_template(
            "Rendering: [{wide_bar:.cyan/blue}] {pos}/{len} Samples ({elapsed} | {eta} Estimated) {last}"
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .with_key("last", move |_: &ProgressState, w: &mut dyn Write| {
                let bits = last_render_clone.load(Ordering::Relaxed);
                let value = f64::from_bits(bits);
                write!(w, "{:.3}s", value).unwrap();
            })
        .progress_chars("=>_"));

    // Main render loop
    for _i in 0..cli_config.samples {
        renderer.render(&arc_scene);
        let time = renderer.get_last_render_time().as_secs_f64();
        last_render.store(time.to_bits(), Ordering::Relaxed);
        pb.inc(1);
    }
    pb.finish();
    let time_elapsed = start_instant.elapsed();
    println!("Rendering took {time_elapsed:?}");

    // Save the render to EXR file
    println!("Saving to {}", cli_config.output);
    renderer.save_exr(&cli_config.output);

}

