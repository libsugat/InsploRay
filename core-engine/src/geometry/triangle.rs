use glam::{Vec2, Vec3};

use crate::acceleration_structure::AABB;
use crate::Ray;
use crate::geometry::{Geometry, HitPayload};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,

    normal: Vec3,
    pub material_id : i32,
}

impl Triangle {
    pub fn new((v0, v1, v2):(Vec3, Vec3, Vec3), matrial: i32) -> Self {
        let normal = (v1 - v0).cross(v2 - v0).normalize();
        Self { v0, v1, v2, normal, material_id : matrial}
    }
}

impl Geometry for Triangle {
    fn intersect_ray(&self, ray :&Ray) -> Option<HitPayload> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        
        let p = ray.direction.cross(e2);

        let det = e1.dot(p);

        if det.abs() < 1e-8 {
            return None;
        }

        let inv_det = 1.0/det.abs();
        let t_vec= ray.origin - self.v0;
        let u = inv_det * t_vec.dot(p);

        if u < 0.0 || u > 1.0 {
            return None;
        }
        
        let q_vec = t_vec.cross(e1);
        let v = inv_det * ray.direction.dot(q_vec);
        
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * e2.dot(q_vec);

        if t < 0.0 {
            return None;
        }

        // let n = if ray.direction.dot(self.normal) > 0.0 {
        let n = if det < 0.0 {
            -self.normal // flip normal to face opposite the ray
        } else {
            self.normal
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
                world_position : ray.origin + t * ray.direction,
                world_normal: n,
                material_index: material,

                uv : Some(Vec2::new(u, v)),
                ..Default::default()
            }
        )
    }

    fn bounding_box(&self) -> AABB {
        let mut min = Vec3::min(self.v0, self.v1);
        min = Vec3::min(min, self.v2);

        let mut max = Vec3::max(self.v0, self.v1);
        max = Vec3::max(max, self.v2);

        // Optional: Expand slightly to avoid degenerate boxes
        let epsilon = 1e-4;
        let min = Vec3::new(min.x - epsilon, min.y - epsilon, min.z - epsilon);
        let max = Vec3::new(max.x + epsilon, max.y + epsilon, max.z + epsilon);

        AABB {
            min: min,
            max: max
        }
    }
}


