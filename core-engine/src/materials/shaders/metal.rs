use glam::Vec3;

use crate::geometry::HitPayload;
use crate::ray::Ray;
use crate::sampler::Sampler;

use super::BxDF;

pub struct Metal {
    pub albedo: Vec3,
}

impl BxDF for Metal {
    fn sample_direction(&self, ray: &Ray, hit_record: &HitPayload, _sampler: &mut Sampler) -> Vec3 {
        ray.direction.reflect(hit_record.world_normal)
    }

    fn eval(&self, wi: Vec3, wo: Vec3, hit_record: &HitPayload) -> Vec3 {
        let normal = hit_record.world_normal;
        if (wi - wo.reflect(normal)).length() < 1e-6 {
            self.albedo
        }
        else {
            Vec3::ZERO
        }
    }

    fn pdf(&self, wo: Vec3, wi: Vec3, hit_record: &HitPayload) -> f32 {
        let normal = hit_record.world_normal;
        // Delta function: probability is 1 for perfect reflection
        if (wi - wo.reflect(normal)).length() < 1e-6 {
            1.0
        } else {
            0.0
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Vec3::ONE,
        }
    }
}
