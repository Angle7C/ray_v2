use std::sync::Arc;

use glam::DVec3;

use crate::pbrt_core::{texture::Texture, bxdf::{specular::SpecularReflection, frensnel::{Fresnel, NoOPFresnel}, BxDF}};

use super::{Material, BSDF};
#[derive(Debug)]
pub struct Mirror{
    kr:Arc<dyn Texture<DVec3>>,

}
impl Mirror{
    pub fn new(kr:Arc<dyn Texture<DVec3>>)->Self{
        Self { kr }
    }
}
impl Material for Mirror{
    fn compute_scattering_functions(&self,suface:&mut crate::pbrt_core::tool::SurfaceInteraction,_mode:crate::pbrt_core::bxdf::TransportMode) {
        let r=self.kr.evaluate(&suface.common).clamp(DVec3::ZERO, DVec3::splat(f64::INFINITY));
        suface.bsdf=Some(BSDF::new(suface, 1.0));
        if let Some(bsdf)=&mut suface.bsdf{
            let fresnel=Fresnel::NoOP(NoOPFresnel{});
            bsdf.bxdfs.push(BxDF::SpecularReflection(SpecularReflection::new(r, fresnel)));
        }
    }
    fn bump(&self,_suface:&crate::pbrt_core::tool::SurfaceInteraction, _texture:& dyn Texture<f64>) {
        
    }
}