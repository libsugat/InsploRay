pub mod base;

pub(crate) mod ray;
pub(crate) mod accumulators;
pub(crate) mod concurrency;
pub(crate) mod integrator;
pub(crate) mod sampler;
pub(crate) mod utils;
pub(crate) mod interations;

pub mod file_formats;
pub mod cameras;
pub mod renderer;
pub mod materials;
pub mod geometry;
pub mod scene;
pub mod accelerators;
pub mod lighting;

// Being experimented With
pub mod bxdfs;

use std::ops::Index;
use std::ops::IndexMut;

use ray::Ray;

pub use glam::Vec2;
pub use glam::Vec3;
pub use glam::Vec4;

pub mod consts {
    pub const EPSILON : f32 = f32::EPSILON * 10.0;
}

#[derive(Default, Debug, Clone)]
pub struct ImageBuffer<PixelData> {
    width: usize,
    height: usize,
    pub buffer: Vec<PixelData>
}

impl<PixelData: Copy + Default> ImageBuffer<PixelData> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![PixelData::default(); width * height]
        }
    }

    pub fn get_dimensions(&self) -> [u32; 2] {
        [self.width as u32, self.height as u32]
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<PixelData> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: PixelData) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = value;
        }
    }

    pub fn map<F>(&mut self, mut f: F)
    where
        F: FnMut(PixelData) -> PixelData,
    {
        for pixel in &mut self.buffer {
            *pixel = f(*pixel);
        }
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = (usize, usize, PixelData)> {
        self.buffer.iter().enumerate().map(move |(i, &pixel)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, pixel)
        })
    }
}


impl<PixelData: Copy + Default> Index<(usize, usize)> for ImageBuffer<PixelData> {
    type Output = PixelData;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.buffer[y * self.width + x]
    }
}

impl<PixelData: Copy + Default> IndexMut<(usize, usize)> for ImageBuffer<PixelData> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.buffer[y * self.width + x]
    }
}
