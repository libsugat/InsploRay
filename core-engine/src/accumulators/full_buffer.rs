use glam::{Vec4};

use super::tile_buffer::TileAccumulator;
use crate::utils::convert_to_argb;

#[derive(Debug, Clone)]
pub struct Accumulator {
    width: u32,
    height: u32,

    framebuffer: Vec<Vec4>,
    sample_counts: Vec<u32>,
}

impl Accumulator {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;

        Self {
            width,
            height,
            framebuffer: vec![Vec4::ZERO; size],
            sample_counts: vec![0; size],
        }
    }

    #[inline]
    pub fn get_resolution(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub fn _accumulate(&mut self, x: u32, y: u32, color: Vec4) {
        debug_assert!(x < self.width && y < self.height, "Pixel out of bounds");

        let index = (y * self.width + x) as usize;

        self.framebuffer[index] += color;
        self.sample_counts[index] += 1;
    }

    pub fn _get_pixel_radiaence(&self, x: u32, y: u32) -> Vec4 {
        debug_assert!(x < self.width && y < self.height, "Pixel out of bounds");

        let index = (y * self.width + x) as usize;
        let color = self.framebuffer[index];
        let samples = self.sample_counts[index].max(1);

        color / samples as f32
    }

    pub fn get_argb_pixel(&self, index: usize) -> u32 {
        let color = self.framebuffer[index];
        let samples = self.sample_counts[index].max(1);

        let mut averaged = color / samples as f32;

        // Tone mapping (Reinhard)
        averaged = averaged / (averaged + Vec4::ONE);

        // Gamma correction
        averaged = averaged.powf(1.0 / 2.2);

        // Clamp to [0, 1]
        averaged = averaged.clamp(Vec4::ZERO, Vec4::ONE);

        convert_to_argb(&averaged)
    }

    #[inline]
    pub fn _get_color_argb(&self, x: u32, y: u32) -> u32 {
        debug_assert!(x < self.width && y < self.height, "Pixel out of bounds");

        self.get_argb_pixel((y * self.width + x) as usize)
    }

    /// Merges two accumulators by summing corresponding pixels and sample counts.
    pub fn _merge(&mut self, b: Self) {
        assert_eq!(
            self.width, b.width,
            "Widths do not match: \nself.width = {:?}\nb.width = {:?}",
            self.width, b.width
        );
        assert_eq!(
            self.height, b.height,
            "Heights do not match: \nself.height = {:?}\nb.height = {:?}",
            self.height, b.height
        );

        for (c1, c2) in self.framebuffer.iter_mut().zip(b.framebuffer) {
            *c1 += c2;
        }

        for (s1, s2) in self.sample_counts.iter_mut().zip(b.sample_counts) {
            *s1 += s2;
        }
    }

    pub fn write_to_image_buffer(&self, buffer: &mut Vec<u32>) {
        if buffer.len() != self.framebuffer.len() {
            *buffer = vec![0xFF000000_u32; self.framebuffer.len()]
        };

        buffer.iter_mut().enumerate().for_each(|(i, pixel)| {
            *pixel = self.get_argb_pixel(i);
        });
    }

    pub fn merge_tile(&mut self, tile: TileAccumulator) {
        for ty in 0..tile.height {
            for tx in 0..tile.width {
                let tile_index = (ty * tile.width + tx) as usize;

                let global_x = tile.offset_x + tx;
                let global_y = tile.offset_y + ty;

                // Safety check (optional)
                debug_assert!(global_x < self.width);
                debug_assert!(global_y < self.height);

                let global_index = (global_y * self.width + global_x) as usize;

                self.framebuffer[global_index] += tile.framebuffer[tile_index];
                self.sample_counts[global_index] += tile.sample_counts[tile_index];
            }
        }
    }
}
