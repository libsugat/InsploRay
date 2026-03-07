use glam::Vec3;

use crate::accelerators::AABB;
use crate::geometry::{Geometry, HitPayload};
use crate::{Ray};

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material_id: i32,
}

impl Geometry for Sphere {
    fn intersect_ray(&self, ray :&Ray) -> Option<HitPayload> {
        let origin = ray.origin - self.position;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(origin);
        let c = origin.dot(origin) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t0 = (-b - sqrt_d) / (2.0 * a);
        let t1 = (-b + sqrt_d) / (2.0 * a);

        let t = if t0 > 0.0 {
            t0
        } else if t1 > 0.0 {
            t1
        } else {return None};

        let hit_point = origin + t * ray.direction;

        let mut normal = hit_point.normalize();
        let mut back_hit = false;

        if normal.dot(-ray.direction) < 0.0 {
            normal = -normal;
            back_hit = true;
        }

        let material = if self.material_id < 0 {
            None
        }
        else {
            Some(self.material_id as usize)
        };

        Some(
            HitPayload {
                hit_distance: t,
                world_position: hit_point + self.position,
                world_normal: normal,
                material_index: material,
                back_hit,
                ..Default::default()
            }
        )
    }

    fn bounding_box(&self) -> AABB {
        let bias_vec = Vec3::from((self.radius, self.radius, self.radius));
        AABB {
            min: self.position - bias_vec,
            max: self.position + bias_vec
        }
    }

    fn centroid(&self) -> Vec3 {
        self.position
    }

}

