use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;

use insploray::materials::{GGXMetal, Material};
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

    println!("Building BVH");
    scene.build_bvh();
    println!("Building BVH Done");
    // scene.spheres.push(insploray::geometry::Sphere { position: Vec3::new(0.0, 4.25, 0.0), radius: 0.2, material_id: -1 });

    let arc_scene = Arc::new(scene);
    println!("Copied to Arc");

    let mut renderer = RayTracer::new(cli_config.width, cli_config.height);
    renderer.set_tp_size(cli_config.nthreads);
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
        print!("{:3}% Done \t", ((i+1) * 100)/cli_config.samples);
    }

    let time_elapsed = start_instance.elapsed();
    renderer.save_exr(&cli_config.output);

    println!("Rendering took took {time_elapsed:?}");
}
