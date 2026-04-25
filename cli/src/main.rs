use std::f32::consts::PI;
use std::fmt::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc};
use std::time::Instant;

use clap::Parser;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use insploray::materials::{DeltaGlass, Material};
use insploray::renderer::RayTracer;
use insploray_cli::CliConfig;

use insploray::Vec3;
use insploray::cameras::PinholeCamera;
use insploray::scene::{Scene, obj_loader};

fn main() {
    // load_scene();
    // build_camera();
    // create_renderer();
    // create_scheduler();
    // render();
    // write_exr();

    // parse_args();
    let cli_config = CliConfig::parse();

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
                    println!("Scene loaded successfully..");
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

    // let mat_1_bsdf = GGXMetal {
    //     base_color: Vec3::new(0.229786, 0.8004, 0.658289),
    //     // base_color: Vec3::new(0.800, 0.435, 0.080),
    //     roughness: 0.4
    // };
    let mat_1_bsdf = DeltaGlass {
        base_color: Vec3::new(1.0, 1.0, 1.0),
        // base_color: Vec3::new(0.800, 0.435, 0.080),
        // roughness: 0.4
        ior: 1.45
    };

    let mat = Arc::new(Material {
        shaders: vec![Arc::new(mat_1_bsdf)],
        weights: vec![1.0],
    });
    scene.materials[3] = mat;

    println!("Building BVH");
    scene.build_bvh();
    println!("Building BVH Done");

    let arc_scene = Arc::new(scene);

    let mut renderer = RayTracer::new(cli_config.width, cli_config.height);
    renderer.set_tile_size(cli_config.tile);
    renderer.set_tp_size(cli_config.nthreads);
    renderer.set_active_camera(Arc::new(cam));
    renderer.update(cli_config.width, cli_config.height);

    let start_instant = Instant::now();
    let last_render = Arc::new(AtomicU64::new(0.0_f64.to_bits()));
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

    for _i in 0..cli_config.samples {
        renderer.render(&arc_scene);
        let time = renderer.get_last_render_time().as_secs_f64();
        last_render.store(time.to_bits(), Ordering::Relaxed);
        pb.inc(1);
    }

    let time_elapsed = start_instant.elapsed();
    renderer.save_exr(&cli_config.output);
    pb.finish();

    println!("Rendering took took {time_elapsed:?}");
}

