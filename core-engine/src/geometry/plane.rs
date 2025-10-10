use glam::Vec3;

use crate::acceleration_structure::AABB;
use crate::geometry::{Geometry, HitPayload};
use crate::Ray;

pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
    pub material_id: i32,
}

impl Geometry for Plane {
    fn intersect_ray(&self, ray :&Ray) -> Option<HitPayload> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < f32::EPSILON {
            return None;
        }

        let num = self.normal.dot(self.position - ray.origin);
        let t = num / denom;

        let facing_normal = if denom < 0.0 {
            self.normal
        }
        else {
            -self.normal
        };

        let material = if self.material_id < 0 {
            None
        }
        else {
            Some(self.material_id as usize)
        };

        Some(
            HitPayload {
                hit_distance: t,
                world_position: ray.origin + t * ray.direction,
                world_normal: facing_normal,
                material_index: material,
                ..Default::default()
            }
        )
    }

    fn bounding_box(&self) -> AABB {
        AABB{
            min: Vec3::ZERO,
            max: Vec3::ONE
        }
    }
}

