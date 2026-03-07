use glam::Vec3;

use crate::{accelerators::AABB, geometry::{Geometry, HitPayload, Triangle}, ray::Ray};

pub struct Mesh {
    pub name: String,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(name: String, triangles: Vec<Triangle>) -> Self {
        Mesh {
            name,
            triangles,
        }

    }
}

impl Geometry for Mesh {
    fn intersect_ray(&self, ray: &Ray) -> Option<HitPayload> {
        let mut hit_distance = f32::MAX;
        let mut closest_hit = HitPayload {
            hit_distance: -1.0,
            ..Default::default()
        };
        
        for triangle in self.triangles.iter() {
            if let Some(payload) = triangle.intersect_ray(ray) {
                if payload.hit_distance > 0.0 && payload.hit_distance < hit_distance {
                    hit_distance = payload.hit_distance;
                    closest_hit = payload;
                }
            }
            else {
                continue;
            }
        }

        Some(closest_hit)
    }

    fn bounding_box(&self) -> AABB {
        let mut aabb = AABB {
            min: Vec3::ZERO,
            max: Vec3::ZERO
        };

        self.triangles.iter().for_each(|tri| {
            aabb = AABB::union_box(aabb, tri.bounding_box());
        });

        aabb
    }

    fn centroid(&self) -> Vec3 {
        if self.triangles.len() == 0 {
            return Vec3::ZERO;
        }

        let mut c = Vec3::ZERO;
        self.triangles.iter().for_each(|tri| {
            c += tri.centroid();
        });

        c / self.triangles.len() as f32
    }

}
