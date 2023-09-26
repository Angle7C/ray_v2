use glam::Vec3A;

use crate::pbrt_core::tool::color::Color;

use super::Texture;

pub struct ConstantTexture{
    value: Color
}
impl ConstantTexture{
    pub fn new(value:Vec3A)->Self{
        let color = Color(value.x,value.y,value.z);
        Self { value: color }
    }
}
impl Texture for ConstantTexture{
    fn evaluate(&self,_common:&crate::pbrt_core::tool::interaction::SurfaceInteraction)->Color {
        self.value
    }
}