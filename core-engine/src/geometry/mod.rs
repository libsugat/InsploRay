use glam::{Vec2, Vec3};

use crate::Ray;
use crate::acceleration_structure::AABB;

#[derive(Default, Debug)]
pub struct HitPayload {
    pub hit_distance: f32,
    pub world_position: Vec3,
    pub world_normal: Vec3,
    pub back_hit: bool,

    pub object_index: Option<usize>,
    pub material_index: Option<usize>,

    // incase of Triangle
    pub uv: Option<Vec2>,
}

pub trait Geometry {
    fn intersect_ray(&self, ray: &Ray) -> Option<HitPayload>;
    fn bounding_box(&self) -> AABB;
    fn centroid(&self) -> Vec3;
}

pub mod sphere;
pub use sphere::Sphere;

pub mod triangle;
pub use triangle::Triangle;

pub mod mesh;
pub use mesh::Mesh;
