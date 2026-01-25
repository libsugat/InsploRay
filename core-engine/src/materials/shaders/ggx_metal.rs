use std::f32::consts::PI;

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::geometry::HitPayload;
use crate::sampler::Sampler;
use crate::utils::{transform_local_to_world};

use super::BxDF;

#[derive(Serialize, Deserialize)]
pub struct GGXMetal {
    pub base_color: Vec3,
    pub roughness: f32,
}

impl BxDF for GGXMetal {
    fn sample_direction(&self, wo: Vec3, hit_record: &HitPayload, sampler: &mut Sampler) -> (Vec3, Vec3) {
        let mut wi = Vec3::ZERO;
        
        // TODO : Move to VNDF (Visual ndf)
       while wi.dot(hit_record.world_normal) <= 0.0 {
            let r1 = sampler.next_f32();
            let r2 = sampler.next_f32();

            let alpha = self.roughness * self.roughness;
            let alpha2 = (alpha * alpha).max(0.000001);

            let cos_theta = ((1.0 - r1) / (1.0 + (alpha2 - 1.0) * r1)).sqrt();
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            let phi = 2.0 * PI * r2;


            let h_local = Vec3::new(
                sin_theta * phi.cos(),
                sin_theta * phi.sin(),
                cos_theta,
            );

            let h_world = transform_local_to_world(h_local, hit_record.world_normal).normalize();
            wi = (-wo).reflect(h_world).normalize();
        }

        (
            wi,
            hit_record.world_normal
        )
    }

    fn eval(&self, wi: Vec3, wo: Vec3, hit_record: &HitPayload) -> Vec3 {
        let n = hit_record.world_normal;

        let ndotwi = n.dot(wi);
        let ndotwo = n.dot(wo);

        if ndotwi <= 0.0 || ndotwo <= 0.0 {
            return Vec3::ZERO;
        }

        let h = (wi + wo).normalize();
        
        let d = self.ndf(h, n);
        let g = self.g_masking(wi, n) * self.g_masking(wo, n);
        // let g = 1.0;
        let f = self.fresnel_schlick(wo, h);

        d * g * f / (4.0 *  ndotwo * ndotwi)
    }

    fn pdf(&self, wi: Vec3, wo: Vec3, hit_record: &HitPayload) -> f32 {
        let h = (wo + wi).normalize();
        let n = hit_record.world_normal;

        let ndotwi = n.dot(wi);
        let ndotwo = n.dot(wo);

        if ndotwi <= 0.0 || ndotwo <= 0.0 {
            return 0.0;
        }

        let ndoth = n.dot(h).max(0.0);
        let wodoth = wo.dot(h).abs();

        // remove division by G_masking after adding VNDF
        self.ndf(h, n) * ndoth / (4.0 * wodoth * self.g_masking(wo, n))
    }
}

impl GGXMetal {
    fn ndf(&self, h: Vec3, n: Vec3) -> f32 {
        let alpha = self.roughness * self.roughness;
        let alpha2 = (alpha * alpha).max(0.000001);
        
        let ndoth = n.dot(h).max(0.0);
        let den = ndoth * ndoth * (alpha2 - 1.0) + 1.0;

        alpha2 / (den * den * PI)
    }

    // G_1(w) term of Smith's G term
    fn g_masking(&self, w: Vec3, n: Vec3) -> f32 {
        let ndotw = n.dot(w).max(0.0);

        let alpha = self.roughness * self.roughness;
        let alpha2 = (alpha * alpha).max(0.000001);

        let root = (alpha2 + (1.0 - alpha2) * ndotw * ndotw).sqrt();
        (2.0 * ndotw) / (ndotw + root)
    }

    // frensel Term or F term
    fn fresnel_schlick(&self, wo: Vec3, h: Vec3) -> Vec3 {
        let f_0 = self.base_color;
        f_0 + (Vec3::ONE - f_0) * (1.0 - wo.dot(h).max(0.0)).powi(5)
    }
}

impl Default for GGXMetal {
    fn default() -> Self {
        Self {
            base_color: Vec3::ONE,
            roughness: 0.5,
        }
    }
}
