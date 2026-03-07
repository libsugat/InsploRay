use glam::Vec3;

use crate::ray::Ray;
pub trait Camera : CameraClone + Send + Sync {
    fn get_ray(&self, x: u32, y: u32) -> Ray;
    fn set_position(&mut self, position: Vec3);
    fn set_rotation(&mut self, rotation: Vec3);
    fn get_image_resolutions(&self) -> [u32; 2];
    fn set_image_resolutions(&mut self, image_resolution: [u32; 2]);

    fn compute_transformation_matrix(&mut self);
    fn on_update(&mut self);
}

pub trait CameraClone {
    fn clone_box(&self) -> Box<dyn Camera + Send + Sync>;
}

impl<T> CameraClone for T
where
    T: Camera + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn Camera + Send + Sync> {
        Box::new(self.clone())
    }
}

