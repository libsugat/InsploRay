use core::f32::consts;

use crate::{Vec3, Vec2};

#[inline(always)]
pub fn sample_uniform_disk_concentric(u: Vec2) -> Vec2 {
    let u_offset = 2.0 * u - Vec2::ONE;
    if u_offset == Vec2::ZERO {
        return Vec2::ZERO;
    }

    let theta: f32;
    let r:f32;
    if u_offset.x.abs() > u_offset.y.abs() {
        r = u_offset.x;
        theta = consts::FRAC_PI_4 * (u.y / u.x);
    }
    else {
        r = u_offset.y;
        theta = consts::FRAC_PI_2 - consts::FRAC_PI_4 * (u.x / u.y);
    }
    return r * Vec2::new(theta.cos(), theta.sin());
}

#[inline(always)]
pub fn sample_cosine_hemisphere(u: Vec2) -> Vec3 {
    let d = sample_uniform_disk_concentric(u);
    let z = (1.0 - d.x * d.x - d.y * d.y).sqrt();
    return Vec3::from((d, z));
}

#[inline(always)]
pub fn cosine_hemisphere_pdf(cos_theta: f32) -> f32 {
    cos_theta * consts::FRAC_1_PI
}
