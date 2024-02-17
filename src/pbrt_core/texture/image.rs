use glam::{Vec2};


use crate::pbrt_core::{filter::Filter, tool::{color::Color, mipmap::MipMap}};

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
    fn evaluate(&self, inter: &crate::pbrt_core::tool::InteractionCommon) -> Color {
        self.mipmap.lookup(inter.uv,Vec2::ZERO,Vec2::ZERO).into()
        
    }
}
