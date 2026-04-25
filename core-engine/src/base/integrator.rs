use crate::{interations::SurfaceInteraction, ray::Ray};

pub struct ShapeIntersection {
    pub intr: SurfaceInteraction,
    pub t_hit: f32,
}

pub trait Integrator {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<ShapeIntersection>;
    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool;
}
