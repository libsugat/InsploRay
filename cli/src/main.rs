use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;

use insploray::renderer::RayTracer;
use insploray_cli::CliConfig;

use insploray::Vec3;
use insploray::cameras::PinholeCamera;
use insploray::scene::{Scene, obj_loader};

fn main() {
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

    println!("Trying loading {}", cli_config.input_file_path);
    match obj_loader::load_from_file(&cli_config.input_file_path) {
        Ok((meshes, materials)) => {
            scene.meshes = meshes;
            scene.materials = materials;
            println!("Scene loaded successfully..");
        }
        Err(e) => {
            println!("Error loading scene : {}", e);
        }
    }
    scene.build_bvh();

    let arc_scene = Arc::new(scene);

    let mut renderer = RayTracer::new(cli_config.width, cli_config.height);
    renderer.set_active_camera(Arc::new(cam));
    renderer.update(cli_config.width, cli_config.height);

    let start_instance = Instant::now();

    for i in 0..cli_config.samples {
        print!("Sample no: {}; ", i);
        renderer.render(&arc_scene);
        println!("{:?}", renderer.get_last_render_time())
    }

    let time_elapsed = start_instance.elapsed();
    renderer.save_exr(&cli_config.output);

    println!("Rendering took took {time_elapsed:?}");
}
