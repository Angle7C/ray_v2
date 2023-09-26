use std::sync::Arc;

use crate::pbrt_core::texture::Texture;

use super::MaterialAble;

pub struct Matte{
    kd: Arc<dyn Texture>,
    simga:f32,
}
impl Matte{
    pub fn new(kd:Arc<dyn Texture>,simga:f32)->Self{
        Self{kd,simga}
    }
}
impl MaterialAble for Matte{
    fn compute(&self, inter: &mut crate::pbrt_core::tool::interaction::SurfaceInteraction) {
        unimplemented!()
    }
}
