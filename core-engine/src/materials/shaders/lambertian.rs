use glam::Vec3;

use crate::geometry::HitPayload;
use crate::sampler::Sampler;

use super::BxDF;
// use super::ScatterRecord

pub struct Lambertian {
    pub albedo: Vec3,
    pub emission_color: Vec3,
    pub emissive_power: f32,
}

impl BxDF for Lambertian {
    fn sample_direction(&self, _wo: Vec3, hit_record: &HitPayload, sampler: &mut Sampler) -> Vec3 {
        let normal = hit_record.world_normal;

        sampler.sample_hemisphere_cosine_weighted(normal).normalize()
    }
    
    fn eval(&self, _wi: Vec3, _wo: Vec3, _hit_record: &HitPayload) -> Vec3 {
        self.albedo / std::f32::consts::PI
    }
    
    fn pdf(&self, wi: Vec3, _wo: Vec3, hit_record: &HitPayload) -> f32 {
        let normal = hit_record.world_normal;
        let cos_theta = wi.dot(normal).max(0.0);
        cos_theta / std::f32::consts::PI
    }
    
    fn emission(&self, _hit_record: &HitPayload) -> Vec3 {
        self.emission_color * self.emissive_power
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Vec3::ONE,
            emission_color: Vec3::ZERO,
            emissive_power: 0.0,
        }
    }
}
