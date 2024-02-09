use std::sync::Arc;

use glam::Vec3;

use crate::pbrt_core::{
    bxdf::{reflection::{LambertianReflection, OrenNayar}, BxDF},
    texture::Texture,
};

use super::{Material, BSDF};
#[derive(Debug)]
pub struct Matte {
    kd: Arc<dyn Texture>,
    _bump: Option<Arc<dyn Texture>>,
    sigma:f32 
}
impl Matte {
    pub fn new(kd: Arc<dyn Texture>,sigma:f32 )-> Self {
        Self {
            kd: kd.clone(),
            sigma,
            _bump: None,
        }
    }
}
impl Material for Matte {
    fn compute_scattering_functions(
        &self,
        suface: &mut crate::pbrt_core::tool::SurfaceInteraction,
        _mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        let r = self
            .kd
            .evaluate(&suface.common)
            .clamp(Vec3::ZERO, Vec3::splat(f32::INFINITY));
        suface.bsdf = Some(BSDF::new(suface, 1.0));
        if let Some(bsdf) = &mut suface.bsdf {
            if r != Vec3::ZERO&&self.sigma==0.0 {
                bsdf.bxdfs
                    .push(BxDF::LambertianReflection(LambertianReflection::new(r)))
            }else{
                bsdf.bxdfs
                    .push(BxDF::OrenNayar(OrenNayar::new(r,self.sigma)))
            }
            
        }
    }

}
