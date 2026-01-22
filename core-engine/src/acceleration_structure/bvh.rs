use std::sync::Arc;

use super::AABB;
use super::AccelerationStructure;
use crate::geometry::Geometry;
use crate::geometry::HitPayload;
use crate::geometry::Triangle;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug)]
pub enum BVHNode {
    Internal {
        bounds: AABB,
        left: Arc<BVHNode>,
        right: Arc<BVHNode>,
    },
    Leaf {
        bounds: AABB,
        primitives: Vec<Triangle>,
    },
}

impl AccelerationStructure for BVHNode {
    fn build(scene: &Scene) -> Option<Self> {
        // Flatten all triangles from all meshes into Geometry objects
        let mut primitives: Vec<Triangle> = Vec::new();
        for mesh in &scene.meshes {
            for tri in &mesh.triangles {
                primitives.push(tri.clone()); 
            }
        }

        if primitives.len() == 0 {
            return None;
        }

        Some(
            Self::build_recursive(primitives)
        )
    }

    fn traverse(&self, ray: &Ray) -> Option<HitPayload> {
        match self {
            BVHNode::Leaf { bounds, primitives } => {
                if bounds.intersect(ray).is_none() {
                    return None;
                }

                let mut hit_distance = f32::MAX;
                let mut closest_hit: Option<HitPayload> = None;

                for prim in primitives.iter() {
                    if let Some(payload) = prim.intersect_ray(ray) {
                        if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                            hit_distance = payload.hit_distance;
                            closest_hit = Some(payload);
                        }
                    }
                    else {
                        continue;
                    }
                }
                
                closest_hit
            },
            BVHNode::Internal { bounds, left, right } => {
                if bounds.intersect(ray).is_none() {
                    return None;
                }

                let left_hit = left.traverse(ray);
                let right_hit = right.traverse(ray);

                match (left_hit, right_hit) {
                    (Some(l_hit), Some(r_hit)) => {
                        if l_hit.hit_distance < r_hit.hit_distance {
                            Some(l_hit)
                        } else {
                            Some(r_hit)
                        }
                    }
                    (Some(hit), None) | (None, Some(hit)) => Some(hit),
                    (None, None) => None,
                }
            }
        }
    }
}

const MAX_PRIMS_IN_NODE: usize = 6;


impl BVHNode {
    fn build_recursive(mut primitives : Vec<Triangle>) -> Self {
        let count = primitives.len();
        if count == 0 {
             return BVHNode::Leaf {
                bounds: AABB::empty(),
                primitives: vec![],
            };
        }

        // compute AABB for all premitives
        let mut bounds = primitives[0].bounding_box();
        for prim in &primitives[1..] {
            bounds = AABB::union_box(bounds, prim.bounding_box());
        }

        if count <= MAX_PRIMS_IN_NODE {
            return BVHNode::Leaf { bounds, primitives: primitives};
        }

        let extent = bounds.max - bounds.min;
        let axis = extent.max_position();

        primitives.sort_by(|a, b| a.centroid()[axis].partial_cmp(&b.centroid()[axis]).unwrap());

        let mid_indx = primitives.len() / 2;

        let left = primitives.split_off(mid_indx);
        let right = primitives;
        

        Self::Internal {
            bounds,
            left: Arc::new(Self::build_recursive(left)),
            right: Arc::new(Self::build_recursive(right)),
        }
    }
}
