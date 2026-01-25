use glam::{Vec3, Vec4};

use crate::acceleration_structure::AccelerationStructure;
use crate::materials::shaders::BxDFImpl;
use crate::{Ray, consts};
use crate::cameras::Camera;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::materials::{Material};

use crate::geometry::{Geometry, HitPayload};

#[derive(Clone, Copy)]
pub struct Integrator {
    pub bounces: usize,
    pub max_compulsory_bounces: usize,
}

impl Integrator {
    pub fn compute_incomming_radience(
        &mut self,
        scene: &Scene,
        x: u32,
        y: u32,
        camera: &dyn Camera,
        sampler: &mut Sampler,
    ) -> Vec4 /* returns radiance per RGB channel */ {
        // let cam = camera.read().unwrap();
        let cam = camera;
        let mut ray = cam.get_ray(x, y);

        let mut light = Vec3::ZERO;

        let default_material: Material = Material {
            shaders : vec![BxDFImpl::default()],
            weights : vec![1.0]
        };
        
        let mut contribution = Vec3::ONE;
        for bounce in 0..self.bounces {
            let payload = self.trace_ray(&ray, scene);

            if payload.hit_distance > 0.0 {
                let material = match payload.material_index {
                    Some(index) => {
                        scene
                            .materials
                            .get(index)
                            .unwrap_or(&default_material)
                    },
                    None => &default_material
                };


                let scatter_data = material.scatter(&ray, &payload, sampler);

                light += scatter_data.emission * contribution;
                let cos_theta = scatter_data.wi.dot(scatter_data.shading_normal).max(0.0);
                contribution *= scatter_data.f / scatter_data.selection_pdf;
                if !scatter_data.is_delta {
                    contribution *= cos_theta / scatter_data.pdf;
                }

                if bounce >= self.max_compulsory_bounces {
                    let p = contribution.x.max(contribution.y.max(contribution.z));
                    if sampler.next_f32() > p {
                        break;
                    }
                    contribution /= p;
                }
                
                ray.origin = payload.world_position + scatter_data.shading_normal * consts::EPSILON;
                ray.direction = scatter_data.wi;
                ray.inv_d = 1.0 / scatter_data.wi;
            } else {
                // sky box, or something
                let sky_color = match &scene.skybox {
                    Some(sky) => sky.light_in_dir(&ray),
                    None => scene.default_sky_color,
                };
                light += sky_color * contribution;
                break;
            }
        }
        Vec4::from((light, 1.0))
    }

    fn trace_ray(&self, ray: &Ray, scene: &Scene) -> HitPayload {
        // (bx^2 + by^2 + bz^2)t^2 + 2(axbx + ayby + azbz)t + (ax^2 + ay^2 + az^2 - r^2)
        // a vec ray origin
        // b vec ray direction
        // r radius
        // t hit distance

        let mut hit_distance = f32::MAX;
        let mut closest_hit = HitPayload {
            hit_distance: -1.0,
            ..Default::default()
        };

        for (_i, sphere) in scene.spheres.iter().enumerate() {
            if let Some(payload) = sphere.intersect_ray(ray) {
                if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                    hit_distance = payload.hit_distance;
                    closest_hit = payload;
                }
            }
            else {
                continue;
            }
        }

        if let Some(bvh) = &scene.bvh {
            if let Some(payload) = bvh.traverse(ray) {
                if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                    closest_hit = payload;
                }
            }
        }

        return closest_hit;
    }
}
