use std::sync::Arc;

use glam::Vec3;

use crate::{geometry::HitPayload, ray::Ray, sampler::Sampler};

pub struct ScatterRecord {
    pub wi: Vec3,
    pub f: Vec3,
    pub pdf: f32,
    pub emission: Vec3,
    pub selection_pdf: f32
}

pub trait BxDF {
    fn sample_direction(&self, wo:Vec3, hit_record: &HitPayload, sampler: &mut Sampler) -> Vec3;
    fn eval(&self, wi: Vec3, wo: Vec3, hit_record: &HitPayload) -> Vec3;
    fn pdf(&self, wi: Vec3, wo: Vec3, hit_record: &HitPayload) -> f32;

    fn emission(&self, _hit_record: &HitPayload) -> Vec3 {
        Vec3::ZERO
    }
}

pub struct Material {
    pub shaders: Vec<Arc<dyn BxDF + Send + Sync>>,
    pub weights: Vec<f32>
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitPayload, sampler: &mut Sampler) -> ScatterRecord {
        if self.shaders.is_empty() {
            // No BxDFs exist — return magenta to indicate missing shader
            return ScatterRecord {
                emission: Vec3::ZERO,
                pdf: 1.0,
                f: Vec3::new(1.0, 0.0, 1.0), // magenta
                wi: Vec3::ZERO,
                selection_pdf: 1.0
            };
        }
        let wo = -ray.direction;

        let weight_sum: f32 = self.weights.iter().sum();
        let random_num = sampler.next_f32() * weight_sum;

        let mut sum:f32 = 0.0;
        for i in 0..self.shaders.len() {
        sum += self.weights[i];
            if random_num < sum {
                let wi = self.shaders[i].sample_direction(wo, hit_record, sampler);
                let f = self.shaders[i].eval(wi, wo, hit_record);
                let pdf = self.shaders[i].pdf(wi, wo, hit_record);
                let emission = self.shaders[i].emission(hit_record);
                
                let p = self.weights[i] / weight_sum;
                return ScatterRecord {
                    wi,
                    f,
                    pdf,
                    emission,
                    selection_pdf: p
                };
            }
        }

        // just an edge case if the generated number is greater than the total weighted sum
        // fallback: pick the last lobe
        let last = self.shaders.len() - 1;
        let bxdf = &self.shaders[last];

        let wi = bxdf.sample_direction(wo, hit_record, sampler);
        let f = bxdf.eval(wo, wi, hit_record);
        let pdf = bxdf.pdf(wo, wi, hit_record);
        let selection_pdf = self.weights[last] / weight_sum;

        ScatterRecord {
            wi,
            f,
            pdf,
            selection_pdf,
            emission: bxdf.emission(hit_record),
        }
    }
}

pub mod shaders;

pub use shaders::lambertian::Lambertian;
pub use shaders::metal::IdealMirror;
pub use shaders::ggx_metal::GGXMetal;
