use glam::{Vec2, Vec3};

use crate::Ray;
use crate::accelerators::AABB;

#[derive(Default, Debug)]
pub struct HitPayload {
    pub hit_distance: f32,
    pub world_position: Vec3,
    pub world_normal: Vec3,
    pub back_hit: bool,

    pub material_index: Option<usize>,

    // incase of Triangle
    pub uv: Vec2,
}

mod triangle;
pub use triangle::TriangleMesh;

pub struct GeometryContext<'a> {
    pub meshes: &'a [TriangleMesh],
}

pub trait Geometry {
    fn intersect_ray(&self, ray: &Ray, ctx: &GeometryContext) -> Option<HitPayload>;
    fn bounding_box(&self, ctx: &GeometryContext) -> AABB;
    fn centroid(&self, ctx: &GeometryContext) -> Vec3;
}

pub mod shapes;

mod sphere;
pub use sphere::Sphere;

pub use triangle::Triangle;

// mod mesh;
// pub use mesh::Mesh;
