use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // normalized
    pub inv_d: Vec3, // 1 / direction
}

#[macro_export]
macro_rules! new_ray {
    ($origin:expr, $unit_dir:expr) => {
        $crate::ray::Ray {
            origin: $origin,
            direction: $unit_dir,
            inv_d: Vec3::ONE / $unit_dir
        }
    };
}
