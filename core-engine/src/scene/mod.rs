use std::sync::Arc;

use glam::Vec3;

use crate::acceleration_structure::BVH;
use crate::file_formats::ExrImage;

use crate::geometry::{Mesh, Sphere};
use crate::materials::shaders::ggx_glossy::Glossy;
use crate::materials::shaders::ggx_metal::GGXMetal;
use crate::materials::{DeltaGlass, Lambertian, Material};

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
        match obj_loader::load_from_file("./assets/models/new_year.obj") {
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
        let mat_2_bsdf = Lambertian {
            albedo: Vec3::new(0.2, 0.2, 0.2), // tweak as desired
            emission_color : Vec3::ONE,
            emissive_power : 10.0,
            ..Default::default()
        };
        scene.materials[2] = Arc::new(Material {
            shaders: vec![Arc::new(mat_2_bsdf)],
            weights: vec![1.0],
        });

        let mat_3_bsdf = Lambertian {
            albedo: Vec3::new(0.2, 0.2, 0.2), // tweak as desired
            emission_color : Vec3::new(0.6, 1.0, 0.558),
            emissive_power : 10.0,
            ..Default::default()
        };
        scene.materials[3] = Arc::new(Material {
            shaders: vec![Arc::new(mat_3_bsdf)],
            weights: vec![1.0],
        });

        let mat_4_bsdf = Lambertian {
            albedo: Vec3::new(0.2, 0.2, 0.2), // tweak as desired
            emission_color : Vec3::new(0.896104, 0.037474, 1.0),
            emissive_power : 10.0,
            ..Default::default()
        };
        scene.materials[4] = Arc::new(Material {
            shaders: vec![Arc::new(mat_4_bsdf)],
            weights: vec![1.0],
        });

        let mat_1_bsdf = GGXMetal {
            base_color: Vec3::new(1.0, 0.637328, 0.301854),
            roughness: 0.463636
        };
        scene.materials[1] = Arc::new(Material {
            shaders: vec![Arc::new(mat_1_bsdf)],
            weights: vec![1.0],
        });

        let mat_5_bsdf = Lambertian {
            albedo: Vec3::new(1.0, 1.0, 1.0),
            // ior: 1.45
            ..Default::default()
        };
        scene.materials[5] = Arc::new(Material {
            shaders: vec![Arc::new(mat_5_bsdf)],
            weights: vec![1.0],
        });

        let mat_6_bsdf = DeltaGlass {
            base_color: Vec3::new(0.281158, 0.635935, 0.801516),
            ior: 1.45
            // roughness: 0.568
        };
        scene.materials[6] = Arc::new(Material {
            shaders: vec![Arc::new(mat_6_bsdf)],
            weights: vec![1.0],
        });

        scene
    }
}

pub mod obj_loader;
pub mod gltf_loader;
