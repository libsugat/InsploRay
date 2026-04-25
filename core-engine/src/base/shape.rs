use crate::interations::SurfaceInteraction;
use crate::{ray::Ray};

pub struct Bounds;
pub struct ShapeSample {
    pub intr: SurfaceInteraction,
    pub pdf: f32,
}

pub trait Shape {
    fn bounds(&self) -> Bounds;
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool;
    fn area(&self) -> f32;
    fn sample(&self, u: crate::Vec2) -> Option<ShapeSample>;
    fn pdf(&self, inter: &SurfaceInteraction) -> f32;
        // PBRT_CPU_GPU inline Float PDF(const ShapeSampleContext &ctx, Vector3f wi) const;

}
