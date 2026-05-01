use std::mem::swap;

use glam::{DVec3, Mat3, Vec3};

use crate::accelerators::AABB;
use crate::{Ray, consts};
use crate::geometry::{Geometry, GeometryContext, HitPayload};

pub struct TriangleMesh {
    pub name: String,
    pub id: u32,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub material_id: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,

    mesh_id: u32
}

impl Triangle {
    pub fn new(v0:u32, v1:u32, v2:u32, mesh_id: u32) -> Self {
        Self { v0, v1, v2, mesh_id}
    }
}

impl Geometry for Triangle {
    fn intersect_ray(&self, ray :&Ray, ctx: &GeometryContext) -> Option<HitPayload> {
        let v0 = ctx.meshes[self.mesh_id as usize].vertices[self.v0 as usize];
        let v1 = ctx.meshes[self.mesh_id as usize].vertices[self.v1 as usize];
        let v2 = ctx.meshes[self.mesh_id as usize].vertices[self.v2 as usize];

        let kz = ray.direction.abs().max_position();
        let mut kx = (kz + 1) % 3;
        let mut ky = (kz + 2) % 3;

        if ray.direction[kz] < 0.0 {
            swap(&mut kx, &mut ky);
        }

        let shear_vec = Vec3::new (-ray.direction[kx] * ray.inv_d[kz],
            -ray.direction[ky] * ray.inv_d[kz],
            ray.inv_d[kz]);

        let mut a = v0 - ray.origin;
        let mut b = v1 - ray.origin;
        let mut c = v2 - ray.origin;

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

        if t < crate::consts::EPSILON * det.abs() {
            return None;
        }

        let rcp_det = 1.0 / det;
        let tf = t * rcp_det * det_sign;
        let hit_u = u * rcp_det;
        let hit_v = v * rcp_det;
        let hit_w = w * rcp_det;

        let n = if ctx.meshes[self.mesh_id as usize].normals.is_empty() {
            (v1 - v0).cross(v2 - v0).normalize() * det_sign
        } else {
            let n0 = ctx.meshes[self.mesh_id as usize].normals[self.v0 as usize];
            let n1 = ctx.meshes[self.mesh_id as usize].normals[self.v1 as usize];
            let n2 = ctx.meshes[self.mesh_id as usize].normals[self.v2 as usize];
            let n = hit_u * n0 + hit_v * n1 + hit_w * n2;
            n.normalize() * det_sign
        };

        Some(
            HitPayload {
                hit_distance: tf,
                world_position : ray.origin + tf * ray.direction,
                world_normal: n,
                material_index: ctx.meshes[self.mesh_id as usize].material_id,

                back_hit,
                ..Default::default()
            }
        )

    }

    fn bounding_box(&self, ctx: &GeometryContext) -> AABB {
        let v0 = ctx.meshes[self.mesh_id as usize].vertices[self.v0 as usize];
        let v1 = ctx.meshes[self.mesh_id as usize].vertices[self.v1 as usize];
        let v2 = ctx.meshes[self.mesh_id as usize].vertices[self.v2 as usize];

        let mut min = Vec3::min(v0, v1);
        min = Vec3::min(min, v2);

        let mut max = Vec3::max(v0, v1);
        max = Vec3::max(max, v2);

        // Optional: Expand slightly to avoid degenerate boxes
        let epsilon = consts::EPSILON;
        let min = Vec3::new(min.x - epsilon, min.y - epsilon, min.z - epsilon);
        let max = Vec3::new(max.x + epsilon, max.y + epsilon, max.z + epsilon);

        AABB {
            min: min,
            max: max
        }
    }

    fn centroid(&self, ctx: &GeometryContext) -> Vec3 {
        let v0 = ctx.meshes[self.mesh_id as usize].vertices[self.v0 as usize];
        let v1 = ctx.meshes[self.mesh_id as usize].vertices[self.v1 as usize];
        let v2 = ctx.meshes[self.mesh_id as usize].vertices[self.v2 as usize];
        (v0 + v1 + v2) / 3.0
    }
}
