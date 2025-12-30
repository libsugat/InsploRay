
use glam::Vec3;

use crate::geometry::HitPayload;
use crate::sampler::Sampler;

use super::BxDF;

pub struct DeltaGlass {
    pub base_color: Vec3,
    pub ior: f32, // Index of Refraction
}

impl BxDF for DeltaGlass {
    fn sample_direction(&self, wo: Vec3, hit_record: &HitPayload, _sampler: &mut Sampler) -> (Vec3, Vec3) {
        let ior = if hit_record.back_hit {
            1.0 / self.ior
        }
        else {
            self.ior
        };

        let mut wi = (-wo).refract(hit_record.world_normal, ior);

        if wi == Vec3::ZERO {
            wi = (-wo).reflect(hit_record.world_normal);
            return (wi, hit_record.world_normal);
        }
        
        (wi, -hit_record.world_normal)
    }

    fn eval(&self, _wi: Vec3, _wo: Vec3, _hit_record: &HitPayload) -> Vec3 {
        // Delta BxDF has zero contribution except for the sampled direction
        self.base_color * self.ior * self.ior
    }

    fn pdf(&self, _wi: Vec3, _wo: Vec3, _hit_record: &HitPayload) -> f32 {
        // Delta BxDF: pdf is zero except in the sampled direction (handled in the renderer)
        // what ever value doesnt matter, never gonna be called if its delta
        1.0
    }

    fn is_delta(&self) -> bool {
        true
    }
}

