use std::sync::Arc;
use super::Texture;
pub struct ScaleTexture{
    tex1:Arc<dyn Texture>,
    tex2:Arc<dyn Texture>
}
impl Texture for ScaleTexture
{
    fn  evaluate(&self,inter:&crate::pbrt_core::tool::InteractionCommon)->glam::Vec3{
        self.tex1.evaluate(inter)*self.tex2.evaluate(inter)        
    }
}
impl std::fmt::Debug for ScaleTexture{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}