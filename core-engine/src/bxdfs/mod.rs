use std::sync::Arc;

use crate::{Vec3, Vec2};

use crate::utils::math::*;
use crate::base::{BSDFSample, BxDF, BxDFFlags, BxDFReflTransFlags, TransportMode};

pub struct BSDF {
    bxdf: Arc<dyn BxDF>,
    shading_frame: Frame
}

impl BSDF {
    pub fn new(ns: Vec3, dpdus: Vec3, bxdf: Arc<dyn BxDF>) -> Self {
        Self {
            bxdf,
            shading_frame: Frame::from_xz(dpdus.normalize(), ns.normalize())
        }
    }

    pub fn flags(&self) -> BxDFFlags {
        self.bxdf.flags()
    }

    pub fn render_to_local(&self, v: Vec3) -> Vec3 {
        self.shading_frame.to_local(v)
    }

    pub fn local_to_render(&self, v: Vec3) -> Vec3 {
        self.shading_frame.from_local(v)
    }

    pub fn f(&self, wo_r: Vec3, wi_r: Vec3, mode: TransportMode) -> Vec3 {
        let (wo, wi) = (self.render_to_local(wo_r), self.render_to_local(wi_r));
        if wo.z == 0.0 {
            return Vec3::ZERO;
        }
        self.bxdf.f(wo, wi, mode)
    }

    pub fn sample_f(&self,
        wo_r: Vec3,
        u: f32,
        u2: Vec2,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags
    ) -> Option<BSDFSample> {
        let wo = self.render_to_local(wo_r);
        if wo.z == 0.0 || self.bxdf.flags() & sample_flags == 0 {
            return None;
        }

        let sample = self.bxdf.sample_f(wo, u, u2, mode, sample_flags);
        match sample {
            None => None,
            Some(mut bs) => {
                if bs.f == Vec3::ZERO || bs.pdf == 0.0 || bs.wi.z == 0.0 {
                    return None;
                }
                bs.wi = self.local_to_render(bs.wi);
                Some(bs)
            }
        } 
    }
    
    pub fn pdf(&self, wo_r: Vec3, wi_r: Vec3, mode: TransportMode, flags: BxDFReflTransFlags) -> f32 {
        let (wo, wi) = (self.render_to_local(wo_r), self.render_to_local(wi_r));
        if wo.z == 0.0 { return 0.0;}
        self.bxdf.pdf(wo, wi, mode, flags)
    }
}

mod diffuse;
pub use diffuse::Diffuse;

mod diffuse_transmissive;
pub use diffuse_transmissive::DiffuseTransmissive;
