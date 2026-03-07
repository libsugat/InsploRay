use std::error::Error;
use std::sync::Arc;

use glam::Vec3;

use crate::accelerators::{AccelerationStructure, BVH};
use crate::file_formats::ExrImage;

use crate::geometry::{Mesh, Sphere};
use crate::lighting::Skybox;
use crate::materials::shaders::ggx_glossy::Glossy;
use crate::materials::{DeltaGlass, Material};

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    // let bvh builder consume, it so that memery uses remain low does not spike
    pub meshes: Option<Vec<Mesh>>,

    pub materials: Vec<Arc<Material>>,
    pub default_sky_color: Vec3,

    pub skybox: Option<Skybox>,
    pub bvh : Option<Arc<dyn AccelerationStructure + Sync + Send>>
    // pub bvh : Option<BVH>
}

impl Scene {


    // This is just something this is not what i want to use, its being used for test
    // consider following code as a config file that changes like a command in cli
    pub fn get_example_scene() -> Self {
        let skybox = match ExrImage::load_exr_image("./assets/env/default_skybox_1_.exr") {
            Err(e) => {
                eprintln!("Failed loading EXR: {}", e);
                None
            },
            Ok(exr) => Some(Skybox::load_for_exr(&exr))
        };

        let mut scene = Self {
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
            let mat_1_bsdf = Glossy {
                // base_color: Vec3::new(1.0, 0.637328, 0.301854),
                base_color: Vec3::ONE,
                roughness: 0.2
            };
            let index = scene.materials.len();
            let _mat_6_bsdf = DeltaGlass {
                base_color: Vec3::new(0.281158, 0.635935, 0.801516),
                ior: 1.45
            };

            let mat = Arc::new(Material {
                shaders: vec![Arc::new(mat_1_bsdf)],
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

    pub fn build_bvh(&mut self) {
        self.bvh = if let Some(bvh) = BVH::build(self) {
            Some(Arc::new(bvh))
        }
        else {
            None
        }
        // self.bvh = BVH::build(self);
    }

    pub fn load_data_form_obj(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        match obj_loader::load_from_file(path) {
            Ok((mut meshes, materials)) => {
                let size = self.materials.len();
                self.materials.extend(materials);

                meshes.iter_mut().for_each(|mesh| {
                    mesh.triangles.iter_mut().for_each(|triangle| {
                        match triangle.material_id {
                            None => (),
                            Some(id) => triangle.material_id = Some(id + size)
                        }
                    });
                });

                self.meshes = Some(meshes);

                Ok(())
            },
            Err(err) => {
                Err(err)
            }
        }
    }
}

pub mod obj_loader;
pub mod gltf_loader;
