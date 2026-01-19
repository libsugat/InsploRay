use std::sync::Arc;
use std::vec;

use super::AABB;
use super::AccelerationStructure;
use crate::geometry::Geometry;
use crate::geometry::HitPayload;
use crate::geometry::Triangle;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::Vec3;


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
                primitives.push(tri.clone()); // Triangle must implement Clone
            }
        }

        if primitives.len() < MAX_PRIMS_IN_NODE {
            let first_prim = primitives.first();
            let mut bounds = match first_prim {
                None => return None,
                Some(prim) => {
                    prim.bounding_box()
                }
            };
            
            for prim in &primitives {
                bounds = AABB::union_box(bounds, prim.bounding_box());
            }

            return Some(
                Self::Leaf { bounds, primitives }
            )
        }

        Some(
            BVHNode::build_recursive(primitives)
        )
    }

    fn traverse(&self, ray: &Ray) -> Option<HitPayload> {
        match self {
            BVHNode::Leaf { bounds, primitives } => {
                if !bounds.intersect(ray) {
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
                if !bounds.intersect(ray) {
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


const MAX_PRIMS_IN_NODE: usize = 4;
const BIN_COUNT: usize = 12;

#[derive(Debug, Clone, Copy)]
struct Bin {
    bounds : AABB,
    count : usize
}

impl BVHNode {
    fn build_recursive(primitives : Vec<Triangle>) -> Self {
        let count = primitives.len();

        // compute AABB for all premitives
        let mut bounds = if count == 0 {
            AABB {
                min : Vec3::ZERO,
                max : Vec3::ZERO
            }
        }
        else {
            primitives[0].bounding_box()
        };
        for prim in &primitives[1..] {
            bounds = AABB::union_box(prim.bounding_box(), bounds);
        }

        if count <= MAX_PRIMS_IN_NODE {
            return BVHNode::Leaf { bounds, primitives: primitives};
        }

        // Compute centroid bounds to determine split axis
        let mut centroid_bounds = AABB {
            min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        };

        for prim in &primitives {
            let centroid = prim.bounding_box().centroid();
            centroid_bounds.min = centroid_bounds.min.min(centroid);
            centroid_bounds.max = centroid_bounds.max.max(centroid);
        }

        let extent = centroid_bounds.max - centroid_bounds.min;

        // Choose the split axis with the largest extent
        let axis = extent.max_position();
        
        if extent[axis] == 0.0 {
            // Degenerate case: make a leaf node
            return BVHNode::Leaf {
                bounds,
                primitives,
            };
        }

        let mut bins = vec![Bin{bounds : AABB::empty(), count : 0}; BIN_COUNT];

        for prim in &primitives {
            let centroid = prim.bounding_box().centroid();
            let bin_index = (((centroid[axis] - centroid_bounds.min[axis]) / extent[axis])
                * BIN_COUNT as f32)
                .clamp(0.0, (BIN_COUNT - 1) as f32) as usize;

            bins[bin_index].bounds = AABB::union_box(
                bins[bin_index].bounds,
                prim.bounding_box(),
            );
            bins[bin_index].count += 1;
        }

        // Precompute prefix sums for SAH
        let mut left_bounds = vec![AABB::empty(); BIN_COUNT - 1];
        let mut right_bounds = vec![AABB::empty(); BIN_COUNT - 1];
        let mut left_counts = vec![0; BIN_COUNT - 1];
        let mut right_counts = vec![0; BIN_COUNT - 1];

        // Left to right
        let mut left_acc_bound = AABB::empty();
        let mut left_acc_count = 0;
        for i in 0..BIN_COUNT - 1 {
            left_acc_bound = AABB::union_box(left_acc_bound, bins[i].bounds);
            left_acc_count += bins[i].count;
            left_bounds[i] = left_acc_bound;
            left_counts[i] = left_acc_count;
        }

        // Right to left
        let mut right_acc_bound = AABB::empty();
        let mut right_acc_count = 0;
        for i in (1..BIN_COUNT).rev() {
            right_acc_bound = AABB::union_box(right_acc_bound, bins[i].bounds);
            right_acc_count += bins[i].count;
            right_bounds[i - 1] = right_acc_bound;
            right_counts[i - 1] = right_acc_count;
        }

        // Find best split
        let mut best_cost = f32::INFINITY;
        let mut best_split = 0;

        for i in 0..BIN_COUNT - 1 {
            if left_counts[i] == 0 || right_counts[i] == 0 {
                continue;
            }

            let cost = left_counts[i] as f32 * left_bounds[i].surface_area()
                + right_counts[i] as f32 * right_bounds[i].surface_area();

            if cost < best_cost {
                best_cost = cost;
                best_split = i;
            }
        }

        // Partition primitives
        let mut left_prims = Vec::new();
        let mut right_prims = Vec::new();

        for prim in primitives {
            let centroid = prim.bounding_box().centroid();
            let bin_index = (((centroid[axis] - centroid_bounds.min[axis]) / extent[axis])
                * (BIN_COUNT - 1) as f32)
                .clamp(0.0, (BIN_COUNT - 1) as f32) as usize;

            if bin_index <= best_split {
                left_prims.push(prim);
            } else {
                right_prims.push(prim);
            }
        }

        // If we fail to split (all on one side), fall back to equal partition
        if left_prims.is_empty() || right_prims.is_empty() {

            let mut all_prims = left_prims;
            all_prims.extend(right_prims); // Combine both

            let mid = all_prims.len() / 2;

            all_prims.sort_by(|a, b| {
                a.bounding_box().centroid()[axis]
                    .partial_cmp(&b.bounding_box().centroid()[axis])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let (left, right): (Vec<_>, Vec<_>) = all_prims
                .into_iter()
                .enumerate()
                .partition(|(i, _)| *i < mid);

            left_prims = left.into_iter().map(|(_, p)| p).collect();
            right_prims = right.into_iter().map(|(_, p)| p).collect();
        }

        BVHNode::Internal {
            bounds,
            left: Arc::new(BVHNode::build_recursive(left_prims)),
            right: Arc::new(BVHNode::build_recursive(right_prims)),
        }

    }
}
