use crate::acceleration_structure::{bvh::BVHNode, AABB};
use crate::geometry::{Geometry, HitPayload, Triangle};
use crate::scene::Scene;
use crate::Ray;

use super::AccelerationStructure;

#[repr(C)]
enum NodeType {
    InnerNode,
    Leaf
}

pub struct BVH {
    node_type: Vec<NodeType>,
    bounds: Vec<AABB>,
    left: Vec<usize>,
    right: Vec<usize>,
    prims_start: Vec<usize>,
    prims_count: Vec<usize>,

    prims: Vec<Triangle>
}

impl AccelerationStructure for BVH {
    fn build(scene: &Scene) -> Option<Self> {
        let mut bvh = BVH {
            node_type: vec![],
            bounds: vec![],
            right: vec![],
            left: vec![],
            prims: vec![],
            prims_start: vec![],
            prims_count: vec![],
        };
        let root = match BVHNode::build(scene) {
            Some(root) => root,
            None => return None,
        };

        let res = bvh.build_recursive(&root);
        assert_eq!(res, 0);
        println!("nodes : {}", bvh.node_type.len());
        Some(bvh)
    }

    fn traverse(&self, ray: &Ray) -> Option<HitPayload> {
        let mut stack = Vec::with_capacity(64);
        stack.push(0);

        let mut hit_distance = f32::MAX;
        let mut closest_hit: Option<HitPayload> = None;

        while let Some(idx) = stack.pop() {
            let bounds = self.bounds.get(idx).unwrap();
            if !bounds.intersect(ray) {
                continue;
            }

            let node = self.node_type.get(idx).unwrap();
            match node {
                NodeType::Leaf => {
                    let start = self.prims_start.get(idx).unwrap().clone();
                    let count = self.prims_count.get(idx).unwrap().clone();

                    let prims = &self.prims[start..start + count];
                    for prim in prims.iter() {

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
                },
                NodeType::InnerNode => {
                    let left = self.left.get(idx).unwrap().clone();
                    let right = self.right.get(idx).unwrap().clone();

                    if left > 0 {
                        stack.push(left as usize);
                    }
                    if right > 0 {
                        stack.push(right as usize);
                    }
                }
            }
        }

        closest_hit
    }
}

impl BVH {
    fn build_recursive(&mut self, root:&BVHNode) -> usize {
        let index = self.node_type.len();
        match root {
            BVHNode::Internal { bounds, left, right } => {
                self.bounds.push(bounds.clone());
                self.node_type.push(NodeType::InnerNode);
                self.prims_start.push(0);
                self.prims_count.push(0);

                self.left.push(0);
                self.right.push(0);

                self.left[index] = self.build_recursive(left);
                self.right[index] = self.build_recursive(right);

                index
            },
            BVHNode::Leaf { bounds, primitives } => {
                self.bounds.push(bounds.clone());
                // self.node_type.push(NodeType::Leaf(primitives.clone()));
                self.node_type.push(NodeType::Leaf);
                let idx_prims_start = self.prims.len();
                self.prims_start.push(idx_prims_start);
                self.prims_count.push(primitives.len());
                self.prims.extend(primitives);

                self.left.push(0);
                self.right.push(0);

                index
            }
        }
    }
}
