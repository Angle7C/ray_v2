use std::fmt::Debug;

use super::Texture;

pub struct ConstantTexture<T>{
    value:T
}
impl<T:Copy> Texture<T> for ConstantTexture<T>{
    fn  evaluate(&self,_inter:&crate::pbrt_core::tool::InteractionCommon)->T {
        self.value
    }
}
impl<T>  ConstantTexture<T>{
    pub fn new(value:T)->Self{
        Self { value }
    }
}
impl<T> Debug for ConstantTexture<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}