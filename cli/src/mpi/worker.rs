use std::sync::Arc;
use std::time::Instant;

use insploray::cameras::{Camera, PinholeCamera};
use mpi::traits::{Communicator, Destination, Source};

use insploray::{Vec3, renderer::RayTracer};
use insploray::acceleration_structure::BVH;
use insploray::lighting::Skybox;
use insploray::materials::Material;
use insploray::scene::Scene;

use crate::mpi::{TAG_JOB, TAG_RESULT, TAG_SHUTDOWN};

use super::utils::{broadcast_data};

pub fn run_worker(world: &impl Communicator, rank: i32, cores: u32) {
    let rp = world.process_at_rank(0);
    rp.send_with_tag(&cores, super::TAG_RESOURCE_INFO);

    // receive data for BVH
    let bvh : Option<BVH> = broadcast_data(&rp, 1, None);

    // receive materials
    let materials : Vec<Material> = broadcast_data(&rp, 1, None);

    // receive camera
    let mut camera : PinholeCamera = broadcast_data(&rp, 1, None);

    // receive sky config
    let (default_sky_color, skyboxdata) : (Vec3, Option<(Vec<Vec3>, [u32; 2])>) 
        = broadcast_data(&rp, 1, None);

    let skybox = match skyboxdata {
        None => None,
        Some((buf, dim)) => {
            Some(Skybox::from_vec3_buff(buf, dim))
        }
    };

    let scene = Scene {
        materials,
        default_sky_color,
        skybox,
        bvh,
        ..Default::default()
    };

    let mut renderer = RayTracer::new(0, 0);
    renderer.set_tp_size(cores as usize);
    
    let arc_scene = Arc::new(scene);

    loop {
        let (data, stat) = rp.receive_vec::<u32>();
        match stat.tag() {
            TAG_JOB => {
                let samples = data[2];
                let width = data[0];
                let height = data[1];

                let start_instance = Instant::now();
                camera.set_image_resolutions([height, width]);
                renderer.set_active_camera(Arc::new(camera.clone()));
                renderer.update(width, height);

                for i in 0..samples {
                    renderer.render(&arc_scene);
                    println!("{:?}", renderer.get_last_render_time());
                    print!("{:3}% Done \t", (i * 100)/samples);
                }
                let time_elapsed = start_instance.elapsed();
                println!("Rank {} : {} samples rendererd in {time_elapsed:?}", rank, samples);
                
                let buf = renderer.get_output_buffer();
                let buffer : &[f32] = insploray::cast_slice(&buf);
                rp.send_with_tag(&buffer[..], TAG_RESULT);
            }, 
            TAG_SHUTDOWN => {
                println!("Received Shutdown message from rank {}", stat.source_rank());
                break;
            },
            _ => ()
        }
    }

}

