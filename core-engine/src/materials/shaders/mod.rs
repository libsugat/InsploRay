use serde::{Deserialize, Serialize};

use super::BxDF;

pub mod lambertian;
pub mod ggx_metal;
pub mod metal;
pub mod delta_glass;
pub mod ggx_glossy;

#[derive(Serialize, Deserialize)]
pub enum BxDFImpl {
    Lambertian(lambertian::Lambertian),
    GGXMetal(ggx_metal::GGXMetal),
    GGXGloss(ggx_glossy::Glossy),
    DeltaMetal(metal::IdealMirror),
    DeltaGass(delta_glass::DeltaGlass)
}

impl BxDF for BxDFImpl {
    fn is_delta(&self) -> bool {
        match self {
            Self::Lambertian(b) => b.is_delta(),
            Self::GGXMetal(b) => b.is_delta(),
            Self::GGXGloss(b) => b.is_delta(),
            Self::DeltaMetal(b) => b.is_delta(),
            Self::DeltaGass(b) => b.is_delta(),
        }
    }

    fn sample_direction(&self, wo:glam::Vec3, hit_record: &crate::geometry::HitPayload, sampler: &mut crate::sampler::Sampler) -> (glam::Vec3, glam::Vec3) {
        match self {
            Self::Lambertian(b) => b.sample_direction(wo, hit_record, sampler),
            Self::GGXMetal(b) => b.sample_direction(wo, hit_record, sampler),
            Self::GGXGloss(b) => b.sample_direction(wo, hit_record, sampler),
            Self::DeltaMetal(b) => b.sample_direction(wo, hit_record, sampler),
            Self::DeltaGass(b) => b.sample_direction(wo, hit_record, sampler),
        }
    }

    fn eval(&self, wi: glam::Vec3, wo: glam::Vec3, hit_record: &crate::geometry::HitPayload) -> glam::Vec3 {
        match self {
            Self::Lambertian(b) => b.eval(wi, wo, hit_record),
            Self::GGXMetal(b) => b.eval(wi, wo, hit_record),
            Self::GGXGloss(b) => b.eval(wi, wo, hit_record),
            Self::DeltaMetal(b) => b.eval(wi, wo, hit_record),
            Self::DeltaGass(b) => b.eval(wi, wo, hit_record),
        }
    }

    fn pdf(&self, wi: glam::Vec3, wo: glam::Vec3, hit_record: &crate::geometry::HitPayload) -> f32 {
        match self {
            Self::Lambertian(b) => b.pdf(wi, wo, hit_record),
            Self::GGXMetal(b) => b.pdf(wi, wo, hit_record),
            Self::GGXGloss(b) => b.pdf(wi, wo, hit_record),
            Self::DeltaMetal(b) => b.pdf(wi, wo, hit_record),
            Self::DeltaGass(b) => b.pdf(wi, wo, hit_record),
        }
    }

    fn emission(&self, hit_record: &crate::geometry::HitPayload) -> glam::Vec3 {
        match self {
            Self::Lambertian(b) => b.emission(hit_record),
            Self::GGXMetal(b) => b.emission(hit_record),
            Self::GGXGloss(b) => b.emission(hit_record),
            Self::DeltaMetal(b) => b.emission(hit_record),
            Self::DeltaGass(b) => b.emission(hit_record),
        }
    }
}

impl Default for BxDFImpl {
    fn default() -> Self {
        Self::Lambertian(lambertian::Lambertian::default())
    }
}

