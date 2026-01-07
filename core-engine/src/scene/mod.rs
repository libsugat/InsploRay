use std::sync::Arc;

use glam::Vec3;

use crate::acceleration_structure::BVH;
use crate::file_formats::ExrImage;

use crate::geometry::{Mesh, Sphere};
use crate::materials::shaders::ggx_glossy::Glossy;
use crate::materials::{DeltaGlass, Material};

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub meshes: Vec<Mesh>,

    pub materials: Vec<Arc<Material>>,
    pub default_sky_color: Vec3,

    pub skybox: Option<ExrImage>,
    pub bvh : Option<BVH>
}

impl Scene {
    // This is just something this is not what i want to use, its being used for test
    // consider following code as a config file that changes like like a commond in cli
    pub fn get_example_scene() -> Self {
        let exr_img = ExrImage::load_exr_image("./assets/env/default_skybox_1_.exr");
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
            // default_sky_color: Vec3::new(0.6, 0.7, 0.9),
            default_sky_color: 0.051 * Vec3::ONE,

            skybox: exr_img.ok(),
            ..Default::default()
        };

        println!("trying loading .obj");
        match obj_loader::load_from_file("./assets/models/Cornell_box.obj") {
            Ok((meshes, materials)) => {
                scene.meshes = meshes;
                scene.materials = materials;
                println!(".obj loaded successfully..");
            },
            Err(_) => {
                println!("error loading .obj");
            }
        }

        println!("Material : {:?}", scene.materials.len());
        {
            let _mat_1_bsdf = Glossy {
                // base_color: Vec3::new(1.0, 0.637328, 0.301854),
                base_color: Vec3::ONE,
                roughness: 0.5
            };
            let index = scene.materials.len();
            let mat_6_bsdf = DeltaGlass {
                base_color: Vec3::new(0.281158, 0.635935, 0.801516),
                // ior: 1.45
                ior: 1.33
                // roughness: 0.568
            };
            let mat = Arc::new(Material {
                shaders: vec![Arc::new(mat_6_bsdf)],
                weights: vec![1.0],
            });
            scene.materials.push(mat);
            let sphere_1 = Sphere {
                position: Vec3::new(0.0, 1.0, 0.0),
                radius: 1.0,
                material_id: index as i32
            };
            scene.spheres.push(sphere_1);
        }
        scene
    }
}

pub mod obj_loader;
pub mod gltf_loader;
