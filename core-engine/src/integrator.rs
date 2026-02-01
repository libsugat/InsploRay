use std::f32::consts::PI;
use std::sync::Arc;

use glam::{Vec3, Vec4};

use crate::acceleration_structure::AccelerationStructure;
use crate::consts::EPSILON;
use crate::{Ray, consts};
use crate::cameras::Camera;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::materials::{BxDF, Lambertian, Material};

use crate::geometry::{Geometry, HitPayload};

#[derive(Clone, Copy)]
pub struct Integrator {
    pub bounces: usize,
    pub max_compulsory_bounces: usize,
}

#[inline]
pub fn luminance(c: Vec3) -> f32 {
    0.2126 * c.x + 0.7152 * c.y + 0.0722 * c.z
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
        let mut ray = camera.get_ray(x, y);

        let mut light = Vec3::ZERO;
        let mut contribution = Vec3::ONE;

        let default_material: Arc<Material> = Arc::new(Material {
            shaders : vec![Arc::new(Lambertian {
                albedo: Vec3::new(1.0, 0.0, 1.0),
                ..Default::default()
            })],
            weights : vec![1.0]
        });

        for bounce in 0..self.bounces {
            let hit = match self.trace_ray(&ray, scene) {
                None => {
                    light += contribution * self.get_no_hit_li(&ray, scene);
                    break;
                },
                Some(pl) => pl
            };

            let material = match hit.material_index {
                Some(index) => {
                    scene
                        .materials
                        .get(index)
                        .unwrap_or(&default_material)
                },
                None => &default_material
            };

            let bxdf = material.sample_brdf(sampler); 
            if !bxdf.is_transmissive() {
                light += contribution * self.compute_direct_light(&ray, scene, &hit, &bxdf);
            }

            // Indirect Lighting
            let scatter_data = material.scatter(&ray, &hit, sampler);

            light += scatter_data.emission * contribution;
            let cos_theta = scatter_data.wi.dot(scatter_data.shading_normal).max(0.0);
            contribution *= scatter_data.f;
            if !scatter_data.is_delta {
                contribution *= cos_theta / scatter_data.pdf;
            }

            if !scatter_data.is_delta && bounce >= self.max_compulsory_bounces {
                let p = luminance(contribution).clamp(0.05, 0.95);
                if sampler.next_f32() > p {
                    break;
                }
                contribution /= p;
            }

            ray.origin = hit.world_position + scatter_data.shading_normal * consts::EPSILON;
            ray.direction = scatter_data.wi;
            ray.inv_d = 1.0 / scatter_data.wi;
        }
        Vec4::from((light, 1.0))
    }

    fn compute_direct_light(&self,
        ray: &Ray,
        scene: &Scene,
        hit : &HitPayload,
        bxdf: &Arc<dyn BxDF>) -> Vec3 {

        // Direct Lighting
        let ligth_intensity = 5.0;
        let light_pos = Vec3::new(2.0, 4.0, 0.0);
        let mut light_dir = hit.world_position - light_pos;
        let distance_from_light = light_dir.length();
        light_dir = light_dir.normalize();

        let shadow_ray = Ray {
            origin: hit.world_position + hit.world_normal * EPSILON,
            direction: -light_dir,
            inv_d: -1.0 / light_dir
        };

        let is_light_visible = match self.trace_ray(&shadow_ray, scene) {
            None => true,
            Some(shadow_h) => shadow_h.hit_distance - EPSILON > distance_from_light
        };

        if !is_light_visible {

        }

        let f = bxdf.eval(-light_dir, -ray.direction, &hit);
        // This only works for non transmissive rightnow
        let cos_theta = (-light_dir).dot(hit.world_normal).max(0.0);

        let li = ligth_intensity / (4.0 * PI * distance_from_light * distance_from_light);
        li * f * cos_theta

    }

    #[inline]
    fn get_no_hit_li(&self, ray: &Ray, scene: &Scene) -> Vec3 {
        // Skybox
        let sky_color = match &scene.skybox {
            Some(sky) => sky.light_in_dir(&ray),
            None => scene.default_sky_color,
        };
        return sky_color;
    }

    fn trace_ray(&self, ray: &Ray, scene: &Scene) -> Option<HitPayload> {

        let mut hit_distance = f32::MAX;
        let mut closest_hit = None;

        for sphere in &scene.spheres {
            if let Some(payload) = sphere.intersect_ray(ray) {
                if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                    hit_distance = payload.hit_distance;
                    closest_hit = Some(payload);
                }
            }
            else {
                continue;
            }
        }

        if let Some(bvh) = &scene.bvh {
            if let Some(payload) = bvh.traverse(ray) {
                if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                    closest_hit = Some(payload);
                }
            }
        }

        closest_hit
    }
}
