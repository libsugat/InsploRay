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
}

impl Camera for PinholeCamera {
    /// this function generated ray directly from world space of camera for performance reason
    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let &[width, height] = &self.image_size;

        let mut vec = Vec2::new(
            (x as f32 + 0.5) / width as f32,
            (y as f32 + 0.5) / height as f32,
        );

        vec = (vec * 2.0 - 1.0) * (self.fov / 2.0).tan();
        vec.x *= self.aspect_ratio;

        let ray_direction = Vec3::new(vec.x, vec.y, -1.0);

        Ray {
            origin: self.position,
            direction: self
                .local_to_world
                .transform_vector3(ray_direction)
                .normalize(),
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
        let rotation = Mat4::from_rotation_z(self.rotation.z)
            * Mat4::from_rotation_y(self.rotation.y)
            * Mat4::from_rotation_x(self.rotation.x);

        let translation = Mat4::from_translation(self.position);

        self.local_to_world = translation * rotation;
    }

    fn on_update(&mut self) {
        self.compute_fov();
        self.compute_transformation_matrix();
        self.compute_camera_directions();
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

    #[test]
    fn transformation_matrix_validation_for_camera() {
        // this is some test data from blender

        // location : <Vector (-2.4027, -2.5716, 3.5259)>
        // rotation : <Euler (x=0.1975, y=-0.7941, z=-1.9074), order='XYZ'>
        // rotation : <bpy_float[4], Object.rotation_axis_angle>
        // World matrix :
        // <Matrix 4x4 (-0.2315,  0.9717, 0.0458, -2.4027)
        //     (-0.6616, -0.1918, 0.7249, -2.5716)
        //     ( 0.7132,  0.1375, 0.6873,  3.5259)
        //     ( 0.0000,  0.0000, 0.0000,  1.0000)>
        // Vertex coordinates for object: Plane
        // Vertex 0:
        // Local:  <Vector (-1.2510, 0.5574, 0.6953)>
        // Global: <Vector (-1.5396, -1.3468, 3.1881)>
        // Vertex 1:
        // Local:  <Vector (-0.3731, -1.2838, 0.5934)>
        // Global: <Vector (-3.5366, -1.6484, 3.4910)>
        // Vertex
        // Local:  <Vector (0.0941, 1.1836, -0.7080)>
        // Global: <Vector (-1.3068, -3.3742, 3.2692)>
        // Vertex 3:
        // Local:  <Vector (1.1367, -0.5054, -0.4624)>
        // Global: <Vector (-3.1782, -3.5619, 3.9493)>
        // Vertex 4:
        // Local:  <Vector (0.2396, 0.5597, 1.4485)>
        // Global: <Vector (-1.8480, -1.7874, 4.7693)>

        // Blender camera transform data
        let position = Vec3::new(-2.4027, -2.5716, 3.5259);
        let rotation = Vec3::new(0.1975, -0.7941, -1.9074); // Euler XYZ in radians

        let camera = PinholeCamera::new(
            position,
            rotation,
            55.0,         // FOV
            35.0,         // Sensor size
            [1920, 1080], // Resolution
        );

        let expected_matrix = Mat4::from_cols_array(&[
            -0.2315, 0.9717, 0.0458, -2.4027, -0.6616, -0.1918, 0.7249, -2.5716, 0.7132, 0.1375,
            0.6873, 3.5259, 0.0000, 0.0000, 0.0000, 1.0000,
        ])
        .transpose();

        let camera_matrix = camera.get_camera_to_world_matrix();

        // Validate that the matrices are close
        assert!(
            expected_matrix.abs_diff_eq(camera_matrix, 1e-4),
            "Camera matrix does not match Blender's world matrix"
        );

        // Vertex local positions (from Blender)
        let local_vertices = [
            Vec3::new(-1.2510, 0.5574, 0.6953),
            Vec3::new(-0.3731, -1.2838, 0.5934),
            Vec3::new(0.0941, 1.1836, -0.7080),
            Vec3::new(1.1367, -0.5054, -0.4624),
            Vec3::new(0.2396, 0.5597, 1.4485),
        ];

        // For each vertex, transform with both matrices and compare
        for (i, local) in local_vertices.iter().enumerate() {
            let transformed_expected = expected_matrix.transform_point3(*local);
            let transformed_camera = camera_matrix.transform_point3(*local);

            assert!(
                transformed_expected.abs_diff_eq(transformed_camera, 1e-4),
                "Vertex {}: mismatch in transformed position.\nExpected: {:?}\nGot: {:?}",
                i,
                transformed_expected,
                transformed_camera
            );
        }
    }

}
