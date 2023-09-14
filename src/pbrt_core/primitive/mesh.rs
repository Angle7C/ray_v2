use glam::{{Vec2, Vec3},Vec4};

#[derive(Debug, Default)]
pub struct Mesh {
    pub point: Vec<Vec3>,
    pub normal: Vec<Vec3>,
    pub tangents: Vec<Vec4>,
    pub uv: Vec<Vec2>,
}
impl Mesh {
    pub fn new(
        point: Vec<Vec3>,
        normal: Vec<Vec3>,
        uv: Vec<Vec2>,
        tangents: Vec<Vec4>,
    ) -> Self {
        Self {
            point,
            normal,
            uv,
            tangents,
        }
    }
}
