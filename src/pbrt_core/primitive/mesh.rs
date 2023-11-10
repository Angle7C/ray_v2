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
    pub fn merge(&mut self, other:&mut Mesh) {
        self.point.append(&mut other.point);
        self.normal.append(&mut other.normal);
        self.uv.append(&mut other.uv);
        self.tangents.append(&mut other.tangents);
    }
    #[inline]
    pub fn pos_size(&self) -> usize {
        self.point.len()
    }
    #[inline]
    pub fn norm_size(&self) -> usize {
        self.normal.len() 
    }
    #[inline]
    pub fn uv_size(&self) -> usize {
        self.uv.len() 
    }
}
