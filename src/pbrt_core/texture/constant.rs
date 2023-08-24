use super::Texture;

pub struct ConstantTexture<T>{
    value:T
}
impl<T:Copy> Texture<T> for ConstantTexture<T>{
    fn  evaluate(&self,_inter:&crate::pbrt_core::tool::InteractionCommon)->T {
        self.value
    }
}