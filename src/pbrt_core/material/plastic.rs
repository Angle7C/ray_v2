use std::sync::Arc;
use glam::Vec3;
use crate::pbrt_core::bxdf::BxDF;
use crate::pbrt_core::bxdf::frensnel::{Fresnel, NoOPFresnel};
use crate::pbrt_core::bxdf::reflection::LambertianReflection;
use crate::pbrt_core::bxdf::specular::SpecularReflection;
use crate::pbrt_core::material::BSDF;

use crate::pbrt_core::texture::Texture;

use super::Material;

#[derive(Debug)]
pub struct Plastic {
    kd: Arc<dyn Texture>,
    ks: Arc<dyn Texture>,
    sigma: Arc<dyn Texture>,
}

impl Plastic {
    pub fn new(kd: Arc<dyn Texture>,
               ks: Arc<dyn Texture>,
               sigma: Arc<dyn Texture>) -> Self {
        Self { kd, ks, sigma }
    }
}

impl Material for Plastic {
    fn compute_scattering_functions(&self, suface: &mut crate::pbrt_core::tool::SurfaceInteraction, mode: crate::pbrt_core::bxdf::TransportMode) {
        let kd = self.kd.evaluate(&suface.common);
        let mut bsdf = BSDF::new(suface, 1.0);
        if kd != Vec3::ZERO {
            bsdf.bxdfs.push(BxDF::LambertianReflection(LambertianReflection::new(kd)));
        }
        let ks = self.ks.evaluate(&suface.common);
        if ks != Vec3::ZERO {
            bsdf.bxdfs.push(BxDF::SpecularReflection(SpecularReflection::new(ks, Fresnel::NoOP(NoOPFresnel))))
        }
    }
}