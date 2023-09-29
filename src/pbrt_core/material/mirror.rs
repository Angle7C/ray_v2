use std::sync::Arc;

use glam::Vec3;

use crate::pbrt_core::{texture::Texture, bxdf::{specular::SpecularReflection, frensnel::{Fresnel, NoOPFresnel}, BxDF}};

use super::{Material, BSDF};
#[derive(Debug)]
pub struct Mirror{
    kr:Arc<dyn Texture>,

}
impl Mirror{
    pub fn new(kr:Arc<dyn Texture>)->Self{
        Self { kr }
    }
}
impl Material for Mirror{
    fn compute_scattering_functions(&self,suface:&mut crate::pbrt_core::tool::SurfaceInteraction,_mode:crate::pbrt_core::bxdf::TransportMode) {
        let r=self.kr.evaluate(&suface.common).clamp(Vec3::ZERO, Vec3::splat(f32::INFINITY));
        suface.bsdf=Some(BSDF::new(suface, 1.0));
        if let Some(bsdf)=&mut suface.bsdf{
            let fresnel=Fresnel::NoOP(NoOPFresnel{});
            bsdf.bxdfs.push(BxDF::SpecularReflection(SpecularReflection::new(r, fresnel)));
        }
    }

}