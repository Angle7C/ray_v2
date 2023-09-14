use glam::Vec3;

use crate::pbrt_core::{filter::Filter, tool::mipmap::MipMap};

use super::Texture;
#[derive(Default, Debug)]
pub struct ImageTexture {
    mipmap: MipMap,
    filter: Filter,
}
impl ImageTexture {
    pub fn new(mipmap: MipMap) -> Self {
        Self {
            mipmap,
            filter: Filter::Repeat,
        }
    }
}

impl Texture for ImageTexture {
    fn evaluate(&self, inter: &crate::pbrt_core::tool::InteractionCommon) -> Vec3 {
        // let uv=self.filter.filter_uv(&inter.uv);
        // let value = self.mipmap.lookup(uv, Vec2::ZERO, Vec2::ZERO);
        // value
        Vec3::ZERO
    }
}
