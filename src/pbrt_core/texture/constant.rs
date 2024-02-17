use std::fmt::Debug;

use glam::Vec3;

use crate::pbrt_core::tool::color::Color;

use super::Texture;

pub struct ConstantTexture{
    value:Color
}
impl Texture for ConstantTexture{
    fn  evaluate(&self,_inter:&crate::pbrt_core::tool::InteractionCommon)->Color {
        self.value
    }
}
impl ConstantTexture{
    pub fn new(value:Color)->Self{
        Self { value }
    }
}
impl Debug for ConstantTexture{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}