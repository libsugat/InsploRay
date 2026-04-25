use std::f32::consts::PI;
use std::mem::swap;
use std::sync::Arc;

use glam::{Mat4, Vec2, Vec3};

use crate::base::shape::{Shape, ShapeSample};
use crate::consts::EPSILON;
use crate::interations::{ShadingData, SurfaceInteraction};
use crate::ray::Ray;

struct QuadricIntersection {
    pub t_hit: f32,
    pub p_obj: Vec3,
    pub phi: f32,
}

pub struct Primitive {
    pub shape: Arc<dyn Shape + Send + Sync>,
    pub material_id: u32,
}

pub struct Sphere {
    pub radius: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub theta_z_min: f32,
    pub theta_z_max: f32,
    pub phi_max: f32,
    render_to_obj: Mat4,
    obj_to_render: Mat4
}

impl Sphere {
    pub fn transform(&mut self, trans: Mat4) {
        self.obj_to_render = trans;
        self.render_to_obj = trans.inverse();
    }

    pub fn init_default() -> Self {
        Sphere {
            radius: 1.0,
            z_min: -1.0,
            z_max: 1.0,
            theta_z_min: 0.0,
            theta_z_max: PI,
            phi_max: 2.0 * PI,
            render_to_obj: Mat4::IDENTITY,
            obj_to_render: Mat4::IDENTITY
        }
    }

    pub fn new(
        radius: f32,
        z_min: f32, z_max: f32,
        theta_z_min: f32, theta_z_max:f32,
        phi_max: f32,
        render_to_obj: Mat4,
        obj_to_render: Mat4
    ) -> Self {
        Self {
            radius,
            z_min, z_max,
            theta_z_min, theta_z_max,
            phi_max, render_to_obj, obj_to_render
        }
    }

    fn basic_intersect(&self, ray: &Ray, t_max: f32) -> Option<QuadricIntersection> {
        let oi = self.render_to_obj.transform_point3(ray.origin);
        let di = self.render_to_obj.transform_vector3(ray.direction);

        let a = di.dot(di);
        let b = 2.0 * di.dot(oi);
        let c = oi.length_squared() - self.radius * self.radius;

        let v = oi - b / (2.0 * a) * di;
        let length = v.length();
        let disciminant = 4.0 * a * (self.radius - length) * (length + self.radius);

        if disciminant < 0.0 {
            return None;
        }

        let sqrt_d = disciminant.sqrt();

        let q = if b < 0.0 {
            -0.5 * (b - sqrt_d)
        } else {
            -0.5 * (b + sqrt_d)
        };

        let mut t0 = q / a;
        let mut t1 = c / q;

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        if t0 > t_max || t1 <= 0.0 {
            return None;
        }
        
        let mut t_shape_hit = t0;
        if t_shape_hit <= 0.0 {
            t_shape_hit = t1;
            if t_shape_hit > t_max {
                return None;
            }
        }

        let mut p_hit = oi + t_shape_hit * di;
        p_hit *= self.radius / p_hit.length();

        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            p_hit.x = EPSILON * self.radius;
        }
        let mut phi = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }

        // Test sphere intersection against clipping params
        if (self.z_min > -self.radius && p_hit.z < self.z_min) || 
            (self.z_min < self.z_max && p_hit.z > self.z_max) || phi > self.phi_max {
            if t_shape_hit == t1 {
                return None;
            }
            if t1 > t_max {
                return None;
            }

            t_shape_hit = t1;

            p_hit = oi + t_shape_hit * di;
            p_hit *= self.radius / p_hit.length();

            if p_hit.x == 0.0 && p_hit.y == 0.0 {
                p_hit.x = EPSILON * self.radius;
            }
            let mut phi = p_hit.y.atan2(p_hit.x);
            if phi < 0.0 {
                phi += 2.0 * PI;
            }
            
            if (self.z_min > -self.radius && p_hit.z < self.z_min) || 
                (self.z_min < self.z_max && p_hit.z > self.z_max) || phi > self.phi_max {
                return None;
            }
        }

        Some(QuadricIntersection {
            t_hit: t_shape_hit,
            p_obj: p_hit,
            phi
        })
    }

    fn interaction_from_intersection(&self, isect: &QuadricIntersection, wo: Vec3) -> SurfaceInteraction {
        let phit = isect.p_obj;
        let phi = isect.phi;
        
        // Calculate parameters
        let cos_theta = phit.z / self.radius;
        let theta = cos_theta.acos();
        let u = phi / self.phi_max;
        let v = (theta - self.theta_z_min) / (self.theta_z_max - self.theta_z_min);

        let z_radius = (phit.x * phit.x + phit.y * phit.y).sqrt();
        let cosphi = phit.x / z_radius;
        let sinphi = phit.y / z_radius;
        let dpdu = Vec3::new(-self.phi_max * phit.y, self.phi_max * phit.x, 0.0);
        let sintheta = (1.0 - cos_theta * cos_theta).sqrt();
        let dpdv = (self.theta_z_max - self.theta_z_min) * Vec3::new(phit.z * cosphi,
            phit.z * sinphi,
            -self.radius * sintheta);

        let d2pduu = -self.phi_max * self.phi_max * Vec3::new(phit.x, phit.y, 0.0);
        let d2pduv =
            (self.theta_z_max - self.theta_z_min) * phit.z * self.phi_max
            * Vec3::new(-sinphi, cosphi, 0.0);
        let d2pdvv = - (self.theta_z_max - self.theta_z_min).powi(2) * phit;

        #[allow(non_snake_case)]
        let E = dpdu.dot(dpdu);
        #[allow(non_snake_case)]
        let F = dpdu.dot(dpdv);
        #[allow(non_snake_case)]
        let G = dpdv.dot(dpdv);

        let n = dpdv.cross(dpdu).normalize();
        let (e, f, g) = (n.dot(d2pduu), n.dot(d2pduv), n.dot(d2pdvv));

        #[allow(non_snake_case)]
        let EGF2 = E.mul_add(G, -F*F);
        #[allow(non_snake_case)]
        let invEFG2 = if EGF2 == 0.0 { 0.0 } else { 1.0/EGF2 };

        let dndu = (f * F - e * G) * invEFG2 * dpdu + (e * F - f * E) * invEFG2 * dpdv;
        let dndv = (g * F - f * G) * invEFG2 * dpdu + (f * F - g * E) * invEFG2 * dpdv;

        // let normal_matrix = self.obj_to_render.inverse().transpose();
        let normal_matrix = self.obj_to_render;
        SurfaceInteraction {
            base: crate::interations::BaseInteraction {
                t: isect.t_hit,
                p: self.obj_to_render.transform_point3(phit),
                wo, 
                n: normal_matrix.transform_vector3(n).normalize(),
                uv: Vec2::new(u, v),
                ..Default::default()
            },
            dpdu: normal_matrix.transform_vector3(dpdu),
            dpdv: normal_matrix.transform_vector3(dpdv),
            dndu: normal_matrix.transform_vector3(dndu),
            dndv: normal_matrix.transform_vector3(dndv),

            shading: ShadingData {
                n: self.obj_to_render.transform_vector3(n),
                dpdu: self.obj_to_render.transform_vector3(dpdu),
                dpdv: self.obj_to_render.transform_vector3(dpdv),
                dndu: self.obj_to_render.transform_vector3(dndu),
                dndv: self.obj_to_render.transform_vector3(dndv),
            },
            ..Default::default()
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        match self.basic_intersect(ray, t_max) {
            None => None,
            Some(isect) => Some(
                self.interaction_from_intersection(&isect, -ray.direction)
            )
        }
    }

    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool {
        self.basic_intersect(ray, t_max).is_some()
    }

    fn sample(&self, _u: crate::Vec2) -> Option<ShapeSample> {
        todo!()
    }
    
    fn pdf(&self, _inter: &SurfaceInteraction) -> f32 {
        1.0 / self.area()
    }
    
    fn bounds(&self) -> crate::base::shape::Bounds {
        todo!()
    }

    fn area(&self) -> f32 {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }
}
