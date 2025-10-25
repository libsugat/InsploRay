use glam::Vec4;

pub struct TileAccumulator {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,

    pub framebuffer: Vec<Vec4>,
    pub sample_counts: Vec<u32>,
}

impl TileAccumulator {
    pub fn new(offset_x: u32, offset_y: u32, width: u32, height: u32) -> Self {
        Self {
            offset_x,
            offset_y,
            width,
            height,
            framebuffer: vec![Vec4::ZERO; (width * height) as usize],
            sample_counts: vec![0; (width * height) as usize],
        }
    }

    pub fn accumulate(&mut self, local_x: u32, local_y: u32, color: Vec4) {
        let index = (local_y * self.width + local_x) as usize;
        self.framebuffer[index] += color;
        self.sample_counts[index] += 1;
    }
}
