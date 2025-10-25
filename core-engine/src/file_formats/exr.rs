use exr::error::Result;
use glam::Vec3;

/// HDR skybox loaded from EXR
#[derive(Default)]
pub struct ExrImage {
    pub pixels_buffer: Vec<Vec3>,
    pub width: usize,
    pub height: usize,
}

impl ExrImage {
    pub fn sample(&self, dir: Vec3) -> Vec3 {
        // let dir = dir.normalize();
        let theta = dir.y.clamp(-1.0, 1.0).acos();
        let phi = dir.z.atan2(dir.x);
        let u = (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        let v = theta / std::f32::consts::PI;

        let x = (u * self.width as f32).floor() as usize % self.width;
        let y = (v * self.height as f32).floor() as usize % self.height;

        self.pixels_buffer[y * self.width + x]
    }

    pub fn load_exr_image(path: &str) -> Result<ExrImage> {
        let image_2d = exr::prelude::read_first_rgba_layer_from_file(
            path,
            |resolution, _| {
                let image_data = vec![Vec3::ZERO; resolution.width() * resolution.height()];
                ExrImage {
                    pixels_buffer: image_data,
                    width: resolution.width(),
                    height: resolution.height(),
                }
            },
            |skybox, pos, (r, g, b, _): (f32, f32, f32, f32)| {
                skybox.pixels_buffer[pos.y() * skybox.width + pos.x()] = Vec3::new(r, g, b);
            },
        );

        match image_2d {
            Ok(img) => {
                let skybox = img.layer_data.channel_data.pixels;
                Ok(skybox)
            }
            Err(err) => Err(err),
        }
    }
}
