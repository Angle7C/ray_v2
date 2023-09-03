use std::sync::Arc;

use glam::DVec3;

use crate::pbrt_core::{
    bxdf::{reflection::LambertianReflection, BxDF},
    texture::Texture,
};

use super::{Material, BSDF};
#[derive(Debug)]
pub struct Matte {
    kd: Arc<dyn Texture<DVec3>>,
    bump: Option<Arc<dyn Texture<f64>>>,
}
impl Matte {
    pub fn new(kd: Arc<dyn Texture<DVec3>>) -> Self {
        Self {
            kd: kd.clone(),
            bump: None,
        }
    }
}
impl Material for Matte {
    fn compute_scattering_functions(
        &self,
        suface: &mut crate::pbrt_core::tool::SurfaceInteraction,
        mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        let r = self
            .kd
            .evaluate(&suface.common)
            .clamp(DVec3::ZERO, DVec3::splat(f64::INFINITY));
        suface.bsdf = Some(BSDF::new(&suface, 1.0));
        if let Some(bsdf) = &mut suface.bsdf {
            if r != DVec3::ZERO {
                bsdf.bxdfs
                    .push(BxDF::LambertianReflection(LambertianReflection::new(r)))
            }
        }
    }

    fn bump(
        &self,
        suface: &crate::pbrt_core::tool::SurfaceInteraction,
        texture: &dyn Texture<f64>,
    ) {
        todo!()
    }
}
