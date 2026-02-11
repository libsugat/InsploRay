use glam::{Vec2, Vec3};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::transform_local_to_world;

pub struct Sampler {
    rng: SmallRng,
}

impl Sampler {
    /// Platform-portable default constructor
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        Self::with_seed(seed)
    }

    pub fn with_seed(seed: u64) -> Self {
        Self { rng: SmallRng::seed_from_u64(seed) }
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.rng.random::<f32>()
    }

    pub fn _next_2d(&mut self) -> Vec2 {
        Vec2::new(self.next_f32(), self.next_f32())
    }

    pub fn _vec_3(&mut self, min: f32, max: f32) -> Vec3 {
        let range = max - min;
        Vec3::new(
            self.next_f32() * range + min,
            self.next_f32() * range + min,
            self.next_f32() * range + min,
        )
    }

    pub fn sample_hemisphere_cosine_weighted(&mut self, normal: Vec3) -> Vec3 {
        let u1 = self.next_f32();
        let u2 = self.next_f32();

        let r = u1.sqrt();
        let phi = 2.0 * std::f32::consts::PI * u2;

        let x = r * phi.cos();
        let y = r * phi.sin();
        let z = (1.0 - u1).sqrt();

        let local_dir = Vec3::new(x, y, z);

        transform_local_to_world(local_dir, normal)
    }
}

impl Default for Sampler {
    fn default() -> Self {
        Self::new()
    }
}

