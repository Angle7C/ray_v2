use glam::DVec4;

use super::{mipmap::MipMap, Texture};
#[derive(Default,Debug)]
pub struct ImageTexture{
    mipmap: MipMap
}
impl ImageTexture{
    pub fn new(mipmap:MipMap)->Self{
        Self { mipmap }
    }
}

impl<T:Into<DVec4>+Copy> Texture<T> for ImageTexture{
    fn  evaluate(&self,inter:&crate::pbrt_core::tool::InteractionCommon)->T {
        todo!()
    }
}
