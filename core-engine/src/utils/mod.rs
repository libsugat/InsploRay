pub mod sampling;
pub mod math;
use crate::{Vec3, Vec4};

pub fn same_hemisphere(w: Vec3, wp: Vec3) -> bool {
    w.z * wp.z > 0.0
}

/// Converts 0.0 to 1.0 to u32 in format 0xAARRGGBB
pub(super) fn convert_to_argb(color: &Vec4) -> u32 /* Results Color */ {
    (((color.w * 255.0) as u32) << 24)
        | (((color.x * 255.0) as u32) << 16)
        | (((color.y * 255.0) as u32) << 8)
        | ((color.z * 255.0) as u32)
}

/// Tansforms local direction into world coordinates
pub fn transform_local_to_world(local_dir: Vec3, normal: Vec3) -> Vec3 {
    let normal = normal.normalize();

    // let a = if normal.x.abs() > 0.99 {
    let a = if normal.x.abs() > normal.y.abs() {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };

    let tangent = normal.cross(a).normalize();
    let bitangent = tangent.cross(normal).normalize();

    tangent * local_dir.x + bitangent * local_dir.y + normal * local_dir.z
}

pub fn _spherical_to_cartesian(theta: f32, phi: f32) -> Vec3 {
    let sin_theta = theta.sin();
    Vec3::new(
        sin_theta * phi.cos(),
        sin_theta * phi.sin(), 
        theta.cos()
    )
}
