use glam::{Vec2, Vec3A};

use super::tool::color::Color;

pub trait BxDFAble {
    fn match_type(&self, rhs: u32) -> bool;
    ///给定两个方向， 返回BSDF值，
    fn f(&self, w0: Vec3A, wi: Vec3A) -> Color;
    fn sample_f(
        &self,
        wo: &Vec3A,
        wi: Vec3A,
        u_point: Vec2,
        pdf: &mut f32,
        sampled_type: Option<&mut BxDFType>,
    ) -> Color;
    fn get_type(&self) -> u32;
    fn pdf(&self, wo: &Vec3A, wi: &Vec3A) -> f32;
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum BxDFType {
    Reflection = 1,
    Transmission = 2,
    Diffuse = 4,
    Glossy = 8,
    Specular = 16,
    All = 31,
}
impl From<BxDFType> for u32 {
    fn from(value: BxDFType) -> Self {
        match value {
            BxDFType::Reflection => 1,
            BxDFType::Transmission => 2,
            BxDFType::Diffuse => 4,
            BxDFType::Glossy => 8,
            BxDFType::Specular => 16,
            BxDFType::All => 31,
        }
    }
}
