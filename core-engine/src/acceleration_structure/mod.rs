use glam::Vec3;

use crate::scene::Scene;
use crate::Ray;
use crate::geometry::HitPayload;

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn intersect(&self, _ray: Ray) -> bool {
        todo!()
    }

    pub fn union_box(_b1: Self, _b2: Self) -> Self {
        todo!()
    }
}

pub trait AcceleratioinStructure {
    fn build(scene: &Scene) -> Self;
    fn traverse(&mut self, ray: Ray) -> HitPayload;
}

pub mod bvh;
pub type BVH = bvh::BVHNode;
