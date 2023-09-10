use glam::{DVec2, DVec3, DVec4};
use log::info;

use crate::pbrt_core::filter::Filter;

use super::{mipmap::MipMap, Texture};
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

impl Texture<DVec3> for ImageTexture {
    fn evaluate(&self, inter: &crate::pbrt_core::tool::InteractionCommon) -> DVec3 {
        let uv=self.filter.filter_uv(&inter.uv);
        let value = self.mipmap.lookup(uv, DVec2::ZERO, DVec2::ZERO);
        value
    }
}
