use glam::Vec3;

use crate::scene::Scene;
use crate::Ray;
use crate::geometry::HitPayload;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn empty() -> Self {
        AABB {
            min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn union_box(b1: Self, b2: Self) -> Self {
        AABB {
            min: b1.min.min(b2.min),
            max: b1.max.max(b2.max)
        }
    }

    #[inline(always)]
    pub fn intersect(&self, ray: &Ray) -> Option<(f32, f32)> {

        // Compute intersection distances for each axis simultaneously
        let t0 = (self.min - ray.origin) * ray.inv_d;
        let t1 = (self.max - ray.origin) * ray.inv_d;

        // Swap where direction is negative
        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        // Horizontal reductions to find max/min over 3 lanes
        let t_enter = tmin.max_element();
        let t_exit = tmax.min_element();

        // Intersection test
        if t_exit >= t_enter && t_exit >= 0.0 {
            Some((t_enter.max(0.0), t_exit))
        }
        else {
            None
        }
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;
        2.0 * (d.x * d.y + d.y * d.z + d.x * d.z)
    }

    pub fn centroid(&self) -> Vec3 {
        0.5 * (self.max + self.min)
    }
}

pub trait AccelerationStructure {
    fn build(scene: &mut Scene) -> Option<Self>
    where 
        Self: Sized;
    fn traverse(&self, ray: &Ray) -> Option<HitPayload>;
}

pub mod bvh;
// pub type BVH = bvh::BVHNode;
pub mod bvh_array;
pub use bvh_array::BVH;
