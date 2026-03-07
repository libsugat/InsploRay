use crate::{ray::Ray};
use crate::base::integrator::ShapeIntersection;

pub struct Bounds;
pub struct ShapeSample;
pub struct Interaction;

pub trait Shape {
    fn bounds(&self) -> Bounds;
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<ShapeIntersection>;
    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool;
    fn area(&self) -> f32;
    fn sample(&self, u: crate::Vec2) -> Option<ShapeSample>;
    fn pdf(&self, inter: &Interaction) -> f32;
        // PBRT_CPU_GPU inline Float PDF(const ShapeSampleContext &ctx, Vector3f wi) const;

}
