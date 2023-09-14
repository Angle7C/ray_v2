use std::sync::Arc;

use glam::Vec3;

use crate::pbrt_core::{
    bxdf::{reflection::LambertianReflection, BxDF},
    texture::Texture,
};

use super::{Material, BSDF};
#[derive(Debug)]
pub struct Matte {
    kd: Arc<dyn Texture>,
    _bump: Option<Arc<dyn Texture>>,
}
impl Matte {
    pub fn new(kd: Arc<dyn Texture>) -> Self {
        Self {
            kd: kd.clone(),
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
        suface.bsdf = Some(BSDF::new(&suface, 1.0));
        if let Some(bsdf) = &mut suface.bsdf {
            if r != Vec3::ZERO {
                bsdf.bxdfs
                    .push(BxDF::LambertianReflection(LambertianReflection::new(r)))
            }
        }
    }

    fn bump(
        &self,
        _suface: &crate::pbrt_core::tool::SurfaceInteraction,
        _texture: &dyn Texture,
    ) {
        todo!()
    }
}
