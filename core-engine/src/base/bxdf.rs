use crate::Vec3;

pub type BxDFReflTransFlags = u32;

pub const BXDF_RT_FLAG_UNSET: BxDFReflTransFlags = 0;
pub const BXDF_RT_FLAG_REFLECTION: BxDFReflTransFlags = 1 << 0;
pub const BXDF_RT_FLAG_TRANSMISSION: BxDFReflTransFlags = 1 << 1;
pub const BXDF_RT_FLAG_ALL: BxDFReflTransFlags = BXDF_RT_FLAG_REFLECTION | BXDF_RT_FLAG_TRANSMISSION;

pub type BxDFFlags = u32;

pub const BXDF_FLAG_UNSET: BxDFFlags = 0;
pub const BXDF_FLAG_REFLECTION: BxDFFlags = 1 << 0; //1
pub const BXDF_FLAG_TRANSMISSION: BxDFFlags = 1 << 1; //2
pub const BSDF_FLAG_DIFFUSE: BxDFFlags = 1 << 2; //4
pub const BSDF_FLAG_GLOSSY: BxDFFlags = 1 << 3; //8
pub const BSDF_FLAG_SPECULAR: BxDFFlags = 1 << 4; //16

pub const BSDF_FLAG_DIFFUSE_REFLECTION: BxDFFlags = BSDF_FLAG_DIFFUSE | BXDF_FLAG_REFLECTION;
pub const BSDF_FLAG_DIFFUSE_TRANSMISSION: BxDFFlags = BSDF_FLAG_DIFFUSE | BXDF_FLAG_TRANSMISSION;
pub const BSDF_FLAG_GLOSSY_REFLECTION: BxDFFlags = BSDF_FLAG_GLOSSY | BXDF_FLAG_REFLECTION;
pub const BSDF_FLAG_GLOSSY_TRANSMISSION: BxDFFlags = BSDF_FLAG_GLOSSY | BXDF_FLAG_TRANSMISSION;
pub const BSDF_FLAG_SPECULAR_REFLECTION: BxDFFlags = BSDF_FLAG_SPECULAR | BXDF_FLAG_REFLECTION;
pub const BSDF_FLAG_SPECULAR_TRANSMISSION: BxDFFlags = BSDF_FLAG_SPECULAR | BXDF_FLAG_TRANSMISSION;

pub const BSDF_FLAG_ALL: BxDFFlags =
    BXDF_FLAG_REFLECTION | BXDF_FLAG_TRANSMISSION | BSDF_FLAG_DIFFUSE | BSDF_FLAG_GLOSSY | BSDF_FLAG_SPECULAR;

#[inline]
pub fn is_reflective(f: BxDFFlags) -> bool {
    f & BXDF_FLAG_REFLECTION != 0
}
#[inline]
pub fn is_transmissive(f: BxDFFlags) -> bool {
    f & BXDF_FLAG_TRANSMISSION != 0
}
#[inline]
pub fn is_diffuse(f: BxDFFlags) -> bool {
    f & BSDF_FLAG_DIFFUSE != 0
}
#[inline]
pub fn is_specular(f: BxDFFlags) -> bool {
    f & BSDF_FLAG_SPECULAR != 0
}
#[inline]
pub fn is_glossy(f: BxDFFlags) -> bool {
    f & BSDF_FLAG_GLOSSY != 0
}
#[inline]
pub fn is_non_specular(f: BxDFFlags) -> bool {
    f & (BSDF_FLAG_DIFFUSE | BSDF_FLAG_GLOSSY) != 0
}

pub enum TransportMode {
    Radiance,
    Importance
}

#[derive(Debug)]
pub struct BSDFSample {
    pub f: Vec3,
    pub wi: Vec3,
    pub pdf: f32,
    pub flags: BxDFFlags,
    pub eta: f32,
    pub pdf_is_proportional: bool
}

pub trait BxDF {
    fn f(&self, wo: Vec3, wi: Vec3, mode: TransportMode) -> Vec3;
    fn sample_f(&self, wo: Vec3, u: f32, uc: f32,
        mode: TransportMode, sample_flags: BxDFReflTransFlags) -> Option<BSDFSample>;
    fn pdf(&self, wo: Vec3, wi: Vec3, mode: TransportMode, sample_flags: BxDFReflTransFlags) -> f32;

}
