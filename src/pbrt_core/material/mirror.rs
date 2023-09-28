use std::sync::Arc;

use crate::pbrt_core::texture::Texture;

use super::MaterialAble;

pub struct Mirror<'a> {
    kr: Arc<dyn Texture+'a>,
}
impl<'a> Mirror<'a> {
    pub fn new(kr: Arc<dyn Texture+'a>) -> Self {
        Self { kr }
    }
}
impl<'a> MaterialAble for Mirror<'a> {
    fn compute(&self, inter: &mut crate::pbrt_core::tool::interaction::SurfaceInteraction) {
        unimplemented!()
    }
}
