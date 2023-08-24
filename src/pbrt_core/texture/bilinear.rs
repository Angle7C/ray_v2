    use std::{sync::Arc, ops::{Mul, Add}};

use super::{Texture, TextureMapping2D};

pub struct  BilinearTexture<T:Copy>{
    mapping:Arc<dyn TextureMapping2D>,
    v00: T,
    v01: T,
    v10: T,
    v11: T
}
impl<T> Texture<T> for BilinearTexture<T>
where T:Copy+Mul<f64,Output = T>+Add<T,Output=T>,
{
    fn  evaluate(&self,inter:&crate::pbrt_core::tool::InteractionCommon)->T {
        let (st,_,_) = self.mapping.map(inter);
        self.v00* (1.0-st.x)*(1.0-st.y)+self.v01*(1.0-st.x)*st.y+
        self.v10* st.x*(1.0-st.y)+self.v11*(st.x*st.y)
    }
}