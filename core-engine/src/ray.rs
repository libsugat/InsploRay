use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // normalized
    pub inv_d: Vec3, // 1 / direction
}
