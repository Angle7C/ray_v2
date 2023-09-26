use std::sync::Arc;

use crate::pbrt_core::texture::Texture;

use super::MaterialAble;

pub struct Mirror {
    kr: Arc<dyn Texture>,
}
impl Mirror {
    pub fn new(kr: Arc<dyn Texture>) -> Self {
        Self { kr }
    }
}
impl MaterialAble for Mirror {
    fn compute(&self, inter: &mut crate::pbrt_core::tool::interaction::SurfaceInteraction) {
        unimplemented!()
    }
}
