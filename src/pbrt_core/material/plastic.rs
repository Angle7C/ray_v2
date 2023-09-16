use std::sync::Arc;

use crate::pbrt_core::texture::Texture;

use super::Material;
#[derive(Debug)]
pub struct Plastic{
    kd:Arc<dyn Texture>,
    ks:Arc<dyn Texture>,
    sigma:Arc<dyn Texture>,
}
impl Plastic{
    pub fn new(    kd:Arc<dyn Texture>,
        ks:Arc<dyn Texture>,
        sigma:Arc<dyn Texture>)->Self{
            Self { kd, ks, sigma }
        }   
}
impl Material for Plastic{
    fn compute_scattering_functions(&self, suface: &mut crate::pbrt_core::tool::SurfaceInteraction, mode: crate::pbrt_core::bxdf::TransportMode) {
        
    }
}