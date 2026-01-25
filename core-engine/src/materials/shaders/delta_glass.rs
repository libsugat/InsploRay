use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::geometry::HitPayload;
use crate::sampler::Sampler;

use super::BxDF;

#[derive(Serialize, Deserialize)]
pub struct DeltaGlass {
    pub base_color: Vec3,
    pub ior: f32, // Index of Refraction
}

impl DeltaGlass {
    fn fresnel_term(&self, eta: f32, cos_theta: f32) -> f32 {
        let mut f_0 = (1.0 - eta) / (1.0 + eta);
        f_0 *= f_0;
        
        f_0 + (1.0 - f_0) * (1.0 - cos_theta).powi(5)
    }
}

impl BxDF for DeltaGlass {
    fn sample_direction(&self, wo: Vec3, hit_record: &HitPayload, sampler: &mut Sampler) -> (Vec3, Vec3) {
        let ior = if hit_record.back_hit {
            self.ior
        }
        else {
            1.0 / self.ior
        };

        let cos_theta = wo.dot(hit_record.world_normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if sin_theta * ior > 1.0 || self.fresnel_term(ior, cos_theta) > sampler.next_f32() {
            let wi = (-wo).reflect(hit_record.world_normal);
            (wi, hit_record.world_normal)
        }
        else {
            let wi = (-wo).refract(hit_record.world_normal, ior);
            (wi, -hit_record.world_normal)
        }
    }

    fn eval(&self, wi: Vec3, wo: Vec3, hit_record: &HitPayload) -> Vec3 {
        let ior = if hit_record.back_hit {
            self.ior
        }
        else {
            1.0 / self.ior
        };

        let is_transmission = wi.dot(wo) < 0.0;
        if is_transmission {
            // TODO : Add transmission energy loss by T term
            // i.e T * ior * ior
            self.base_color * ior * ior
        }
        else {
            Vec3::ONE
        }
    }

    fn pdf(&self, _wi: Vec3, _wo: Vec3, _hit_record: &HitPayload) -> f32 {
        0.0
    }

    fn is_delta(&self) -> bool {
        true
    }
}
