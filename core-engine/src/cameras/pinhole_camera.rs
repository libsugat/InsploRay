use glam::{Mat4, Vec2, Vec3, Vec4Swizzles};

pub use super::Camera;
use crate::Ray;

#[derive(Debug, Default, Clone)]
pub struct PinholeCamera {
    pub position: Vec3,
    pub rotation: Vec3, // [x, y, z] Eular rotation in radians

    pub image_size: [u32; 2], // image resolutions
    pub focal_length: f32,
    pub sensor_size: f32, // camera film size

    // cached data
    aspect_ratio: f32,
    local_to_world: Mat4,
    pub forward: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    ray_cache: Vec<Ray>,

    pub fov: f32, // again radians, optained by focal length and sensor size
}

impl PinholeCamera {
    pub fn new(
        position: Vec3,
        rotation: Vec3,
        focal_length: f32,
        sensor_size: f32,
        image_size: [u32; 2],
    ) -> Self {
        let mut camera = Self {
            position,
            rotation,
            focal_length,
            sensor_size,
            ..Default::default()
        };

        camera.set_image_resolutions(image_size);

        camera.on_update();

        camera
    }

    pub fn set_focal_length(&mut self, focal_length: f32) {
        self.focal_length = focal_length;
        self.compute_fov();
    }

    pub fn set_sensor_size(&mut self, sensor_size: f32) {
        self.sensor_size = sensor_size;
        self.compute_fov();
    }

    #[inline]
    fn compute_fov(&mut self) {
        self.fov = 2.0 * ((self.sensor_size / (2.0 * self.focal_length)).atan());
    }

    fn compute_camera_directions(&mut self) {
        self.up = self.local_to_world.y_axis.xyz();
        self.right = self.local_to_world.x_axis.xyz();
        self.forward = self
            .local_to_world
            .transform_vector3(Vec3::new(0.0, 0.0, -1.0));
    }

    pub fn get_camera_to_world_matrix(&self) -> Mat4 {
        self.local_to_world
    }

    fn generate_ray_cache(&mut self) {
        for y in 0..self.image_size[1] {
            for x in 0..self.image_size[0] {
                self.ray_cache.push(self.generate_ray(x, y));
            }
        }
    }

    fn generate_ray(&self, x: u32, y: u32) -> Ray {
        let &[width, height] = &self.image_size;

        let mut vec = Vec2::new(
            (x as f32 + 0.5) / width as f32,
            (y as f32 + 0.5) / height as f32,
        );

        vec = (vec * 2.0 - 1.0) * (self.fov / 2.0).tan();
        vec.x *= self.aspect_ratio;

        let ray_direction = Vec3::new(vec.x, vec.y, -1.0);
        let ray_dir_global = self
                .local_to_world
                .transform_vector3(ray_direction)
                .normalize();

        Ray {
            origin: self.position,
            direction: ray_dir_global,
            inv_d: 1.0 / ray_dir_global
        }
    }

}

impl Camera for PinholeCamera {
    /// this function generated ray directly from world space of camera for performance reason
    fn get_ray(&self, x: u32, y: u32) -> Ray {
        match self.ray_cache.get((y * self.image_size[0] + x) as usize) {
            Some(ray) => ray.clone(),
            None => {
                self.generate_ray(x, y)
            },
        }
    }


    fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.on_update();
    }

    fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
        self.on_update();
    }

    fn set_image_resolutions(&mut self, image_resolution: [u32; 2]) {
        self.image_size = image_resolution;
        self.aspect_ratio = self.image_size[0] as f32 / self.image_size[1] as f32;
    }

    fn get_image_resolutions(&self) -> [u32; 2] {
        self.image_size
    }

    fn compute_transformation_matrix(&mut self) {
        let rotation = Mat4::from_rotation_x(self.rotation.x)
            * Mat4::from_rotation_y(self.rotation.y)
            * Mat4::from_rotation_z(self.rotation.z);

        let translation = Mat4::from_translation(self.position);

        self.local_to_world = translation * rotation;
    }


    fn on_update(&mut self) {
        self.compute_fov();
        self.compute_transformation_matrix();
        self.compute_camera_directions();
        self.generate_ray_cache();
    }
}

#[cfg(test)]
mod test {
    use std::f32::EPSILON;

    use super::*;

    #[test]
    fn fov_calculation() {
        let focal_length = 35.0;
        let sensor_size = 55.0;

        let camera = PinholeCamera::new(
            Vec3::ZERO,
            Vec3::ZERO,
            focal_length,
            sensor_size,
            [1920, 1080],
        );

        let expected_fov = 2.0 * (sensor_size / (2.0 * focal_length)).atan();

        // Allow a small floating-point error margin
        assert!(
            (camera.fov - expected_fov).abs() < EPSILON,
            "Expected FOV: {}, got: {}",
            expected_fov,
            camera.fov
        );
    }


}
