use std::sync::Arc;

use super::AABB;
use super::AccelerationStructure;
use crate::geometry::Geometry;
use crate::geometry::GeometryContext;
use crate::geometry::HitPayload;
use crate::geometry::Triangle;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone)]
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
    fn build(scene: &mut Scene) -> Option<Self> {
        // Flatten all triangles from all meshes into Geometry objects
        let primitives = match scene.tris_vec.take() {
            None => {
                return None;
            },
            Some(x) => x
        };

        if primitives.len() == 0 {
            return None;
        }
        let g_ctx = scene.create_context();

        Some(
            Self::build_recursive(primitives, &g_ctx)
        )
    }

    fn intersect(&self, ray: &Ray, g_ctx: &GeometryContext) -> Option<HitPayload> {
        match self {
            BVHNode::Leaf { bounds, primitives } => {
                if bounds.intersect(ray).is_none() {
                    return None;
                }

                let mut hit_distance = f32::MAX;
                let mut closest_hit: Option<HitPayload> = None;

                for prim in primitives.iter() {
                    if let Some(payload) = prim.intersect_ray(ray, g_ctx) {
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

                let left_hit = left.intersect(ray, g_ctx);
                let right_hit = right.intersect(ray, g_ctx);

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

const MAX_PRIMS_IN_NODE: usize = 8;
const N_BUCKETS: usize = 64;

#[derive(Default, Clone, Copy)]
struct SAHSplitBucket {
    pub count: u32,
    pub bounds: AABB
}

impl BVHNode {
    fn build_recursive(mut primitives : Vec<Triangle>, g_ctx: &GeometryContext) -> Self {
        let count = primitives.len();
        if count == 0 {
             return BVHNode::Leaf {
                bounds: AABB::empty(),
                primitives: vec![],
            };
        }

        // compute AABB for all premitives
        let mut bounds = primitives[0].bounding_box(g_ctx);
        for prim in &primitives[1..] {
            bounds = AABB::union_box(bounds, prim.bounding_box(g_ctx));
        }

        if count <= MAX_PRIMS_IN_NODE {
            return BVHNode::Leaf { bounds, primitives: primitives};
        }

        let mut centroid_bound = AABB::empty();
        for prims in &primitives {
            centroid_bound = AABB::union(centroid_bound, prims.bounding_box(g_ctx).centroid());
        }
        centroid_bound.min -= crate::consts::EPSILON;
        centroid_bound.max += crate::consts::EPSILON;

        let axis = centroid_bound.max_dim();

        if centroid_bound.max[axis] == centroid_bound.min[axis] {
            return BVHNode::Leaf { bounds, primitives: primitives};
        }

        let mut left: Vec<Triangle> = vec![];
        let mut right: Vec<Triangle> = vec![];

        // SAH Code starts here
        if primitives.len() <= 2 {
            let mid = primitives.len() / 2;
            primitives.select_nth_unstable_by(mid, |a, b| 
                a.centroid(g_ctx)[axis].partial_cmp(&b.bounding_box(g_ctx).centroid()[axis]).unwrap()
            );
        }
        else {
            const N_BUCKETS_F32: f32 = N_BUCKETS as f32;
            let mut buckets : [SAHSplitBucket; N_BUCKETS] = [SAHSplitBucket {
                count: 0, bounds: AABB::empty()
            }; N_BUCKETS]; 

            for prim in &primitives {
                let mut b = (N_BUCKETS_F32 * centroid_bound.offset(
                    prim.bounding_box(g_ctx).centroid()
                )[axis]).abs() as usize;
                if b == N_BUCKETS {
                    b = N_BUCKETS - 1;
                }
                if !(b < N_BUCKETS) {
                    println!("b: {}; coetroid_bound: {:?}; offset: {:?}\ncentroid: {:?}; axis: {};",
                        b, centroid_bound, centroid_bound.offset(prim.bounding_box(g_ctx).centroid()), prim.bounding_box(g_ctx).centroid(), axis);
                    println!("b < N_BUCKETS so creating leaf");
                }
                assert!(b < N_BUCKETS);
                buckets[b].count += 1;
                buckets[b].bounds = AABB::union_box(buckets[b].bounds, prim.bounding_box(g_ctx));
            }

            const N_SPLITS: usize = N_BUCKETS - 1;
            let mut costs: [f32; N_SPLITS] = [0.0; N_SPLITS];

            let mut count_below = 0;
            let mut bound_below = AABB::empty();
            for i in 0..N_SPLITS {
                bound_below = AABB::union_box(bound_below, buckets[i].bounds);
                count_below += buckets[i].count;
                costs[i] = count_below as f32 * bound_below.surface_area();
            }

            let mut count_above = 0;
            let mut bound_above = AABB::empty();
            for i in (1..=N_SPLITS).rev() {
                bound_above = AABB::union_box(bound_above, buckets[i].bounds);
                count_above += buckets[i].count;
                costs[i - 1] += count_above as f32 * bound_above.surface_area();
            }

            let mut min_cost_bucket = 0usize;
            let mut min_cost = costs[0];

            for i in 1..N_SPLITS {
                if costs[i] < min_cost {
                    min_cost = costs[i];
                    min_cost_bucket = i;
                }
            }


            let leaf_cost = primitives.len() as f32;
            min_cost = 0.5 + min_cost / bounds.surface_area();

            if primitives.len() > MAX_PRIMS_IN_NODE || min_cost < leaf_cost {
                let (right_c, left_c): (Vec<_>, Vec<_>) = primitives
                    .into_iter()
                    .partition(|a| {
                        let mut b = (N_BUCKETS_F32 * centroid_bound.offset(a.bounding_box(g_ctx).centroid())[axis]) as usize;
                        if b == N_BUCKETS {
                            b = N_BUCKETS - 1;
                        }

                        b <= min_cost_bucket
                    });
                right = right_c;
                left = left_c;
            }
            else {
                return BVHNode::Leaf { bounds, primitives: primitives};
            }
        }

        if left.is_empty() {
            return BVHNode::Leaf { bounds, primitives: right};
        }

        if right.is_empty() {
            return BVHNode::Leaf { bounds, primitives: left};
        }

        Self::Internal {
            bounds,
            left: Arc::new(Self::build_recursive(left, g_ctx)),
            right: Arc::new(Self::build_recursive(right, g_ctx)),
        }
    }
}
