use glam::Vec3;

use crate::acceleration_structure::BVH;
use crate::file_formats::ExrImage;

use crate::geometry::{Mesh, Sphere};
use crate::materials::{Material};

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub meshes: Vec<Mesh>,

    pub materials: Vec<Material>,
    pub default_sky_color: Vec3,

    pub skybox: Option<ExrImage>,
    pub bvh : Option<BVH>
}

impl Scene {
    pub fn get_example_scene() -> Self {
        let exr_img = ExrImage::load_exr_image("./assets/env/docklands_02_1k_.exr");
        if let Err(e) = &exr_img {
            println!(
                "Current working directory: {}",
                std::env::current_dir().unwrap().display()
            );
            eprintln!("Failed loading EXR: {}", e);
        }

        let mut scene = Self {
            spheres: vec![],
            materials: vec![],
            default_sky_color: Vec3::new(0.6, 0.7, 0.9),

            skybox: exr_img.ok(),
            ..Default::default()
        };

        println!("trying loading .obj");
        match obj_loader::load_from_file("./assets/models/test_scene2.obj") {
        // match obj_loader::load_from_file("./assets/models/cornell_box_only_box.obj") {
            Ok((meshes, materials)) => {
                scene.meshes = meshes;
                scene.materials = materials;
                println!(".obj loaded successfully..");
            },
            Err(_) => {
                println!("error loading .obj");
            }
        }

        {
            let material = Material {
                albedo: Vec3::new(1.0, 0.0, 1.0),
                // emission_color: Vec3::new(1.0, 0.0, 1.0),
                emission_color: Vec3::ONE,
                emissive_power: 5.0,
                ..Default::default()
            };
            
            let index = scene.materials.len();
            scene.materials.push(material);

            let sphere = Sphere {
                position: Vec3::new(0.0, 5.0, 0.0),
                radius: 1.0,
                material_id: index as i32,
            };
            scene.spheres.push(sphere);
        }

        scene
    }
}

pub mod obj_loader;
pub mod gltf_loader;
