use std::sync::Arc;

use super::AABB;
use super::AcceleratioinStructure;
use crate::geometry::Geometry;
use crate::ray::Ray;
use crate::scene::Scene;

pub enum BVHNode {
    Internal {
        bounds : AABB,
        left : Arc<BVHNode>,
        right : Arc<BVHNode>
    }, 
    Leaf {
        bounds : AABB,
        primitives : Vec<Box<dyn Geometry>>
    }
}

impl AcceleratioinStructure for BVHNode {
    fn build(_scene: &Scene) -> Self {
        todo!()
    }

    fn traverse(&mut self, _ray : Ray) -> crate::geometry::HitPayload {
        todo!()
    }
}
