use std::fmt::Debug;

use glam::Vec3;

use super::Texture;

pub struct ConstantTexture{
    value:Vec3
}
impl Texture for ConstantTexture{
    fn  evaluate(&self,_inter:&crate::pbrt_core::tool::InteractionCommon)->Vec3 {
        self.value
    }
}
impl ConstantTexture{
    pub fn new(value:Vec3)->Self{
        Self { value }
    }
}
impl Debug for ConstantTexture{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}