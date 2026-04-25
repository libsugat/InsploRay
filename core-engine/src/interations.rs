use glam::{Vec2, Vec3};

#[derive(Default)]
pub struct BaseInteraction {
    pub t: f32,
    pub p: Vec3,
    pub wo: Vec3,
    pub n: Vec3,
    pub time: f32,
    pub uv: Vec2,
    // pub medium_interface: Option(MediumInterface),
    // pub medium: Medium
}

#[derive(Default)]
pub struct ShadingData {
    pub n: Vec3,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Vec3,
    pub dndv: Vec3,
}

#[derive(Default)]
pub struct SurfaceInteraction {
    pub base: BaseInteraction, 
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Vec3,
    pub dndv: Vec3,
    pub shading: ShadingData,

    pub face_index: usize,
    pub material: u32,
    
    pub dpdx: Vec3,
    pub dpdy: Vec3,
    pub dudx: f32,
    pub dudy: f32,
    pub dvdx: f32,
    pub dvdp: f32,
}

// pub struct MediumInteraction {
//     pub base: BaseInteraction,
//     pub phase: PhaseFunction
// }
