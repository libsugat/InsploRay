use crate::{Vec2, Vec3};

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

impl std::ops::Not for TransportMode {
    type Output = TransportMode;
    fn not(self) -> Self::Output {
        match self {
            Self::Radiance => Self::Importance,
            Self::Importance => Self::Radiance
        }
    }
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

impl BSDFSample {
    #[inline(always)]
    pub fn new(f: Vec3, wi: Vec3, pdf: f32, flags: BxDFFlags, eta: f32) -> Self {
        BSDFSample { f, wi, pdf, flags, eta, pdf_is_proportional: false }
    }
}

impl Default for BSDFSample {
    fn default() -> Self {
        BSDFSample {
            f: Vec3::default(), // Initializes to (0.0, 0.0, 0.0) if Vec3 implements Default
            wi: Vec3::default(), // Same for wi
            pdf: 0.0,
            flags: BxDFFlags::default(),
            eta: 1.0, // Default eta value as specified
            pdf_is_proportional: false, // Default bool value as specified
        }
    }
}

impl BSDFSample {
    pub fn is_reflective(&self) -> bool {
        is_reflective(self.flags)
    }
    pub fn is_transissive(&self) -> bool {
        is_transmissive(self.flags)
    }
    pub fn is_diffuse(&self) -> bool {
        is_diffuse(self.flags)
    }
    pub fn is_glossy(&self) -> bool {
        is_glossy(self.flags)
    }
    pub fn is_specular(&self) -> bool {
        is_specular(self.flags)
    }
    pub fn is_non_specular(&self) -> bool {
        is_non_specular(self.flags)
    }
}

pub trait BxDF {
    fn f(&self, wo: Vec3, wi: Vec3, mode: TransportMode) -> Vec3;
    fn sample_f(&self, wo: Vec3, uc: f32, u: Vec2,
        mode: TransportMode, sample_flags: BxDFReflTransFlags) -> Option<BSDFSample>;
    fn pdf(&self, wo: Vec3, wi: Vec3, mode: TransportMode, sample_flags: BxDFReflTransFlags) -> f32;
    fn flags(&self) -> BxDFFlags;

    /*
    // BxDF Method Definitions
SampledSpectrum BxDF::rho(Vector3f wo, pstd::span<const Float> uc,
                          pstd::span<const Point2f> u2) const {
    if (wo.z == 0)
        return {};
    SampledSpectrum r(0.);
    for (size_t i = 0; i < uc.size(); ++i) {
        // Compute estimate of $\rho_\roman{hd}$
        pstd::optional<BSDFSample> bs = Sample_f(wo, uc[i], u2[i]);
        if (bs && bs->pdf > 0)
            r += bs->f * AbsCosTheta(bs->wi) / bs->pdf;
    }
    return r / uc.size();
}

SampledSpectrum BxDF::rho(pstd::span<const Point2f> u1, pstd::span<const Float> uc,
                          pstd::span<const Point2f> u2) const {
    SampledSpectrum r(0.f);
    for (size_t i = 0; i < uc.size(); ++i) {
        // Compute estimate of $\rho_\roman{hh}$
        Vector3f wo = SampleUniformHemisphere(u1[i]);
        if (wo.z == 0)
            continue;
        Float pdfo = UniformHemispherePDF();
        pstd::optional<BSDFSample> bs = Sample_f(wo, uc[i], u2[i]);
        if (bs && bs->pdf > 0)
            r += bs->f * AbsCosTheta(bs->wi) * AbsCosTheta(wo) / (pdfo * bs->pdf);
    }
    return r / (Pi * uc.size());
}
    */

}
