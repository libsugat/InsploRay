use crate::{geometry::{Geometry, HitPayload, Triangle}, ray::Ray};

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
        
        for (_i, sphere) in self.triangles.iter().enumerate() {
            if let Some(payload) = sphere.intersect_ray(ray) {
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
}
