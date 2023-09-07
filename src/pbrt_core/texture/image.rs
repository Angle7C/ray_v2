use glam::{DVec4, DVec2, DVec3};
use log::info;

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

impl Texture<DVec3> for ImageTexture{
    fn evaluate(&self,inter:&crate::pbrt_core::tool::InteractionCommon)->DVec3 {
       let value= self.mipmap.lookup(inter.uv, DVec2::ZERO, DVec2::ZERO);
       value
    }
}
