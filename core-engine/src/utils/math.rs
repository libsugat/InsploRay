use crate::Vec3;

pub struct Frame {
    x: Vec3,
    y: Vec3,
    z: Vec3
}

#[allow(unused)]
impl Frame {
    pub fn from_xy(x: Vec3, y: Vec3) -> Self {
        Self { x, y, z : x.cross(y) }
    }

    pub fn from_xz(x: Vec3, z: Vec3) -> Self {
        Self { x, y: z.cross(x), z }
    }

    pub fn to_local(&self, v: Vec3) -> Vec3 {
        Vec3::new(v.dot(self.x), v.dot(self.y), v.dot(self.z))
    }

    pub fn from_local(&self, v: Vec3) -> Vec3 {
        v.x * self.x + v.y * self.y + v.z * self.z
    }
}
