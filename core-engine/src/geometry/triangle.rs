use std::mem::swap;

use glam::{DVec3, Mat3, Vec3};

use crate::accelerators::AABB;
use crate::{Ray, consts};
use crate::geometry::{Geometry, HitPayload};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,

    pub material_id : Option<usize>,
}

impl Triangle {
    pub fn new((v0, v1, v2):(Vec3, Vec3, Vec3), matrial: Option<usize>) -> Self {
        Self { v0, v1, v2, material_id : matrial}
    }
}

impl Geometry for Triangle {
    fn intersect_ray(&self, ray :&Ray) -> Option<HitPayload> {

        let kz = ray.direction.abs().max_position();
        let mut kx = (kz + 1) % 3;
        let mut ky = (kz + 2) % 3;

        if ray.direction[kz] < 0.0 {
            swap(&mut kx, &mut ky);
        }

        let shear_vec = Vec3::new (-ray.direction[kx] * ray.inv_d[kz],
            -ray.direction[ky] * ray.inv_d[kz],
            ray.inv_d[kz]);

        let mut a = self.v0 - ray.origin;
        let mut b = self.v1 - ray.origin;
        let mut c = self.v2 - ray.origin;
        a = Vec3::new(a[kx], a[ky], a[kz]);
        b = Vec3::new(b[kx], b[ky], b[kz]);
        c = Vec3::new(c[kx], c[ky], c[kz]);

        let m = Mat3::from_cols(Vec3::X, Vec3::Y, shear_vec);

        let ap = m * a;
        let bp = m * b;
        let cp = m * c;

        let mut u = cp.x * bp.y - cp.y * bp.x; 
        let mut v = ap.x * cp.y - ap.y * cp.x; 
        let mut w = bp.x * ap.y - bp.y * ap.x; 

        if u == 0.0 || v == 0.0 || w == 0.0 {
            let apd = DVec3::from(ap);
            let bpd = DVec3::from(bp);
            let cpd = DVec3::from(cp);

            let ud = cpd.x * bpd.y - cpd.y * bpd.x; 
            let vd = apd.x * cpd.y - apd.y * cpd.x; 
            let wd = bpd.x * apd.y - bpd.y * apd.x; 

            u = ud as f32;
            v = vd as f32;
            w = wd as f32;
        } 

        let has_neg = (u < 0.0) | (v < 0.0) | (w < 0.0);
        let has_pos = (u > 0.0) | (v > 0.0) | (w > 0.0);

        if has_neg & has_pos {
            return None;
        }

        let det = u + v + w;

        if det == 0.0 {
            return None;
        }

        let det_sign = det.signum();
        let back_hit = det_sign == -1.0;
        let t = (u * ap.z + bp.z * v + w * cp.z) * det_sign; 

        // if t < 0.0 || t > hit.t * det {
        if t < crate::consts::EPSILON * det.abs() {
            return None;
        }

        let rcp_det = 1.0 / det;
        let tf = t * rcp_det * det_sign;
        // let hit_u = u * rcp_det;
        // let hit_v = v * rcp_det;

        let n = (self.v1 - self.v0).cross(self.v2 - self.v0).normalize() * det_sign;

        Some(
            HitPayload {
                hit_distance: tf,
                world_position : ray.origin + tf * ray.direction,
                world_normal: n,
                material_index: self.material_id,

                back_hit,
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
        let epsilon = consts::EPSILON;
        let min = Vec3::new(min.x - epsilon, min.y - epsilon, min.z - epsilon);
        let max = Vec3::new(max.x + epsilon, max.y + epsilon, max.z + epsilon);

        AABB {
            min: min,
            max: max
        }
    }

    fn centroid(&self) -> Vec3 {
        (self.v0 + self.v1 + self.v2) / 3.0
    }
}
