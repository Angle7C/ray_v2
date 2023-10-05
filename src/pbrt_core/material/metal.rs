use std::sync::Arc;

use glam::Vec3;

use crate::pbrt_core::{texture::Texture, material::BSDF, bxdf::{microfacet_distribution::{roughness_to_alpha, TrowbridgeReitzDistribution}, frensnel::{Fresnel, ConductorFresnel}, reflection::MicrofacetReflection}};

use super::Material;
#[derive(Debug)]
pub struct MetalMaterial{
    //折射率
    eta: Arc<dyn Texture>,
    // 金属率
    k: Arc<dyn Texture>,
    // roughness ,u_roughness,v_roughness
    // 粗糙度
    roughness:Arc<dyn Texture>,

    remap:bool
}
impl MetalMaterial {
    pub fn new(  eta: Arc<dyn Texture>,
        // 金属率
        k: Arc<dyn Texture>,
        // roughness ,u_roughness,v_roughness
        // 粗糙度
        roughness:Arc<dyn Texture>,remap:bool)->Self{
            Self { eta, k, roughness, remap }

    }
}
impl Material for MetalMaterial{
    
    

    fn compute_scattering_functions(&self, surface: &mut crate::pbrt_core::tool::SurfaceInteraction, _mode: crate::pbrt_core::bxdf::TransportMode) {

        let mut bsdf=BSDF::new(&surface, 1.0);
        let roughness=self.roughness.evaluate(&surface.common);
        let (u_alpha,v_alpha)=(roughness_to_alpha(roughness.y),roughness_to_alpha(roughness.z));
        let eta=self.eta.evaluate(&surface.common);
        let k=self.k.evaluate(&surface.common);
        let fresnel=Fresnel::Conductor(ConductorFresnel::new(Vec3::ONE, eta, k));
        let distrib=TrowbridgeReitzDistribution::new(u_alpha, v_alpha, false);
        let bxdf = MicrofacetReflection::new(Vec3::ONE, Box::new(distrib), fresnel);
        bsdf.bxdfs.push(crate::pbrt_core::bxdf::BxDF::MicrofacetReflection(bxdf));
        surface.bsdf=Some(bsdf)
    }

}