use glam::Vec3;

use crate::{file_formats::ExrImage, ray::Ray, ImageBuffer};

pub struct Skybox(ImageBuffer<Vec3>);

impl Skybox {
    pub fn light_in_dir(&self, ray: &Ray) -> Vec3 {
        let dir = ray.direction;
        let theta = dir.y.clamp(-1.0, 1.0).acos();
        let phi = dir.z.atan2(dir.x);
        let u = (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        let v = theta / std::f32::consts::PI;

        let x = (u * self.0.width as f32).floor() as usize % self.0.width;
        let y = (v * self.0.height as f32).floor() as usize % self.0.height;

        self.0.buffer[y * self.0.width + x]
    }

    pub fn load_for_exr(exr: &ExrImage) -> Self {
        Self(exr.rgb.clone())
    }
}
