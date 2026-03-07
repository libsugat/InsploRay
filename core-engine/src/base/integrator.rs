use crate::ray::Ray;

pub struct ShapeIntersection;

pub trait Integrator {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<ShapeIntersection>;
    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool;
}
