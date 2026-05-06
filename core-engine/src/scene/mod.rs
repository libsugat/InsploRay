use std::error::Error;
use std::sync::Arc;

use glam::Vec3;

use crate::accelerators::{AccelerationStructure, BVH};

use crate::geometry::shapes::Primitive;
use crate::geometry::{HitPayload, Triangle};
use crate::geometry::{GeometryContext, TriangleMesh};
use crate::lighting::Skybox;
use crate::materials::{ Material};
use crate::ray::Ray;

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Primitive>,
    pub meshes : Vec<TriangleMesh>,
    pub tris_vec: Option<Vec<Triangle>>,

    pub materials: Vec<Arc<Material>>,
    pub default_sky_color: Vec3,

    pub skybox: Option<Skybox>,
    pub bvh : Option<Arc<dyn AccelerationStructure + Sync + Send>>
}

impl Scene {
    pub fn intersect(&self, ray : &Ray) -> Option<HitPayload>{
        let mut hit_distance = f32::MAX;
        let mut closest_hit = None;

        for sphere in &self.spheres {
            if let Some(interaction) = sphere.shape.intersect(ray, f32::MAX) {
                if interaction.base.t < hit_distance {
                    hit_distance = interaction.base.t;
                    closest_hit = Some(HitPayload {
                        hit_distance,
                        world_position: interaction.base.p,
                        world_normal: interaction.shading.n,
                        back_hit: ray.direction.dot(interaction.base.n) > 0.0,
                        material_index: Some(sphere.material_id as usize),
                        uv: interaction.base.uv,
                        ..Default::default()
                    })
                }
            }
        }

        if let Some(bvh) = &self.bvh {
            let g_ctx = self.create_context();
            if let Some(payload) = bvh.intersect(ray, &g_ctx) {
                if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                    closest_hit = Some(payload);
                }
            }
        }

        closest_hit
    }


    pub fn create_context(&self) -> GeometryContext<'_> {
        GeometryContext { meshes: &self.meshes }
    }

    pub fn build_bvh(&mut self) {
        self.bvh = if let Some(bvh) = BVH::build(self) {
            Some(Arc::new(bvh))
        }
        else {
            None
        }
    }

    pub fn load_data_form_obj(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        match obj_loader::load_from_obj(self, path) {
            Ok(()) => {
                Ok(())
            },
            Err(err) => {
                Err(err)
            }
        }
    }
}

pub mod obj_loader;

mod example_scene;
pub use example_scene::get_example_scene;
