use std::sync::Arc;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use crate::Vec3;
use crate::Mat4;

use crate::geometry::shapes::Sphere;
use crate::geometry::shapes::Primitive;
use crate::materials::{Lambertian, Material};
use super::Scene;
use crate::file_formats::ExrImage; 
use crate::lighting::Skybox;


/// This is just something this is not what i want to use, its being used for test
/// consider following code as a config file that changes like a command in cli
pub fn get_example_scene() -> Scene {
    let skybox = match ExrImage::load_exr_image("./assets/env/default_skybox_1_.exr") {
        Err(e) => {
            eprintln!("Failed loading EXR: {}", e);
            None
        },
        Ok(exr) => Some(Skybox::load_for_exr(&exr))
    };

    let mut scene = Scene {
        spheres: vec![],
        materials: vec![],
        // default_sky_color: Vec3::new(0.6, 0.7, 0.9),
        default_sky_color: 0.051 * Vec3::ONE,

        skybox: skybox,
        ..Default::default()
    };

    match scene.load_data_form_obj("./assets/models/Cornell_box.obj") {
        // match scene.load_data_form_obj("./../InsploRayMemorialScenes/Bunny.obj ") {
        Ok(_) => println!("Example Scene loaded successfully"),
        Err(e) => println!("Error loading file : {:?}", e)
    }

    println!("Material : {:?}", scene.materials.len());
    {
        let mat_1_bsdf = Lambertian {
            albedo: Vec3::new(1.0, 0.637328, 0.301854),
            // albedo: Vec3::ONE * 0.9,
            ..Default::default()
        };
        let index = scene.materials.len();

        let mat = Arc::new(Material {
            shaders: vec![Arc::new(mat_1_bsdf)],
            weights: vec![1.0],
        });
        scene.materials.push(mat);
        // let sphere_1 = Sphere {
        //     position: Vec3::new(0.0, 1.0, 0.0),
        //     radius: 1.0,
        //     material_id: index as i32
        // };
        let position = Vec3::new(0.0, 1.0, 0.0);
        let mat = Mat4::from_translation(position) * Mat4::from_rotation_x(FRAC_PI_2)* Mat4::from_rotation_z(0.0) * Mat4::from_rotation_y(PI);
        let mut sphere_1 = Sphere::init_default();
        sphere_1.transform(mat);
        // sphere_1.z_max = sphere_1.radius * 0.75;
        // sphere_1.z_min = - sphere_1.radius * 0.75;
        sphere_1.phi_max = FRAC_PI_4 * 1.5;
        scene.spheres.push(Primitive {
            shape: Arc::new(sphere_1),
            material_id: index as u32
        });

        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = x.cross(y);

        println!("{:?}", z);
    }
    scene
}
