use std::f32::consts::{FRAC_1_PI};

use crate::{Vec3, Vec2};
use crate::base::bxdf::*;
use crate::utils::same_hemisphere;

use crate::utils::sampling;

pub struct Diffuse {
    albedo: Vec3
}

impl BxDF for Diffuse {
    fn f(&self, wo: Vec3, wi: Vec3, _mode: TransportMode) -> Vec3 {
        if !same_hemisphere(wo, wi) {
            return Vec3::ZERO;
        }
        self.albedo * FRAC_1_PI
    }

    fn sample_f(&self, wo: Vec3, _uc: f32, u: Vec2,
            _mode: TransportMode, sample_flags: BxDFReflTransFlags) -> Option<BSDFSample> {
        if sample_flags & BXDF_RT_FLAG_REFLECTION == 0 {
            return None;
        }
        let mut wi = sampling::sample_cosine_hemisphere(u);
        if wo.z < 0.0 {
            wi *= -1.0;
        }
        let pdf = sampling::cosine_hemisphere_pdf(wi.cos().z.abs());
        Some(BSDFSample {
            f: self.albedo * FRAC_1_PI,
            wi,
            pdf,
            flags: BSDF_FLAG_DIFFUSE_REFLECTION,
            ..Default::default()
        })
    }

    fn pdf(&self, wo: Vec3, wi: Vec3, _mode: TransportMode, sample_flags: BxDFReflTransFlags) -> f32 {
        if sample_flags & BXDF_RT_FLAG_REFLECTION == 0 || !same_hemisphere(wo, wi) {
            return 0.0;
        }
        sampling::cosine_hemisphere_pdf(wi.cos().z.abs())
    }

    fn flags(&self) -> BxDFFlags {
        if self.albedo == Vec3::ZERO {
            BXDF_FLAG_UNSET
        } 
        else {
            BXDF_FLAG_REFLECTION
        }
    }
}
