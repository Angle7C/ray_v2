use std::sync::Arc;

use crate::pbrt_core::texture::Texture;

use super::MaterialAble;

pub struct Matte<'a>{
    kd: Arc<dyn Texture+'a>,
    simga:f32,
}
impl<'a> Matte<'a>{
    pub fn new(kd:Arc<dyn Texture+'a>,simga:f32)->Self{
        Self{kd,simga}
    }
}
impl<'a> MaterialAble for Matte<'a>{
    fn compute(&self, inter: &mut crate::pbrt_core::tool::interaction::SurfaceInteraction) {
        unimplemented!()
    }
}
