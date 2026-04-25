use std::f32::consts::{FRAC_1_PI};

use crate::{Vec3, Vec2};
use crate::base::bxdf::*;
use crate::utils::same_hemisphere;

use crate::utils::sampling;

pub struct DiffuseTransmissive {
    r: Vec3,
    t: Vec3
}

impl BxDF for DiffuseTransmissive {
    fn f(&self, wo: Vec3, wi: Vec3, _mode: TransportMode) -> Vec3 {
        if !same_hemisphere(wo, wi) {
            self.r * FRAC_1_PI
        }
        else {
            self.t * FRAC_1_PI
        }
    }

    fn sample_f(&self, wo: Vec3, uc: f32, u: Vec2,
            _mode: TransportMode, sample_flags: BxDFReflTransFlags) -> Option<BSDFSample> {
        let mut pr = self.r.max_element();
        let mut pt = self.t.max_element();

        if sample_flags & BXDF_RT_FLAG_REFLECTION == 0 {
            pr = 0.0;
        }
        if sample_flags & BXDF_RT_FLAG_TRANSMISSION == 0 {
            pt = 0.0;
        }
        if pr == 0.0 && pt == 0.0 {
            return None;
        }

        if uc < pr/(pr + pt) {
            let mut wi = sampling::sample_cosine_hemisphere(u);
            if wo.z < 0.0 {
                wi *= -1.0;
            }
            let pdf = sampling::cosine_hemisphere_pdf(wi.cos().z.abs() * pr / (pr + pt));
            Some(BSDFSample {
                f: self.r * FRAC_1_PI,
                wi,
                pdf,
                flags: BSDF_FLAG_DIFFUSE_REFLECTION,
                ..Default::default()
            })
        }
        else {
            let mut wi = sampling::sample_cosine_hemisphere(u);
            if wo.z > 0.0 {
                wi *= -1.0;
            }
            let pdf = sampling::cosine_hemisphere_pdf(wi.cos().z.abs() * pt / (pr + pt));
            Some(BSDFSample {
                f: self.r * FRAC_1_PI,
                wi,
                pdf,
                flags: BSDF_FLAG_DIFFUSE_TRANSMISSION,
                ..Default::default()
            })
        }
    }

    fn pdf(&self, wo: Vec3, wi: Vec3, _mode: TransportMode, sample_flags: BxDFReflTransFlags) -> f32 {
        let mut pr = self.r.max_element();
        let mut pt = self.t.max_element();

        if sample_flags & BXDF_RT_FLAG_REFLECTION == 0 {
            pr = 0.0;
        }
        if sample_flags & BXDF_RT_FLAG_TRANSMISSION == 0 {
            pt = 0.0;
        }
        if pr == 0.0 && pt == 0.0 {
            return 0.0;
        }

        if same_hemisphere(wi, wo) {
            sampling::cosine_hemisphere_pdf(wi.cos().z.abs()) * pr / (pr + pt)
        }
        else {
            sampling::cosine_hemisphere_pdf(wi.cos().z.abs()) * pt / (pr + pt)
        }
        
    }

    fn flags(&self) -> BxDFFlags {
        (if self.r == Vec3::ZERO { BXDF_FLAG_UNSET } else { BXDF_FLAG_REFLECTION }) | 
        (if self.t == Vec3::ZERO { BXDF_FLAG_UNSET } else { BXDF_FLAG_TRANSMISSION })
    }
    
}
