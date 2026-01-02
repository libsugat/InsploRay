use glam::Vec3;

use crate::geometry::HitPayload;
use crate::sampler::Sampler;

use super::BxDF;

pub struct IdealMirror {
    pub base_color: Vec3,
}

impl BxDF for IdealMirror {
    fn sample_direction(&self, wo: Vec3, hit_record: &HitPayload, _sampler: &mut Sampler) -> (Vec3, Vec3) {
        (
            (-wo).reflect(hit_record.world_normal), 
            hit_record.world_normal
        )
    }

    fn eval(&self, wi: Vec3, wo: Vec3, hit_record: &HitPayload) -> Vec3 {
        let normal = hit_record.world_normal;
        if (wi - wo.reflect(normal)).length() < 1e-6 {
            self.base_color
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

impl Default for IdealMirror {
    fn default() -> Self {
        Self {
            base_color: Vec3::ONE,
        }
    }
}
