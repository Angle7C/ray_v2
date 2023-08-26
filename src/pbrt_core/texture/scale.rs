use std::{sync::Arc, ops::Mul};

use super::Texture;

pub struct ScaleTexture<T1:Copy,T2:Copy>{
    tex1:Arc<dyn Texture<T1>>,
    tex2:Arc<dyn Texture<T2>>
}
impl<T1,T2> Texture<T2> for ScaleTexture<T1,T2>
where T1:Copy+Mul<T2> + Mul<T2, Output = T2>,
      T2:Copy+Mul<T1>
{
    fn  evaluate(&self,inter:&crate::pbrt_core::tool::InteractionCommon)->T2 {
        self.tex1.evaluate(inter)*self.tex2.evaluate(inter)        
    }
}
impl<T1:Copy,T2:Copy> std::fmt::Debug for ScaleTexture<T1,T2>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}