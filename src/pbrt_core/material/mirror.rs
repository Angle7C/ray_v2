use std::sync::Arc;

use crate::pbrt_core::{texture::Texture, bxdf::specular::SpecularReflection};

use super::Material;

pub struct Mirror{
    kr:Arc<dyn Texture<f64>>,

}
impl Material for Mirror{
    fn compute_scattering_functions(&self,suface:&mut crate::pbrt_core::tool::SurfaceInteraction,mode:crate::pbrt_core::bxdf::TransportMode) {
        let r=self.kr.evaluate(suface).clamp(0.0, 1.0);
        suface.bsdf=Some(SpecularReflection)
    }
    fn bump(&self,suface:&crate::pbrt_core::tool::SurfaceInteraction, texture:& dyn Texture<f64>) {
        
    }
}