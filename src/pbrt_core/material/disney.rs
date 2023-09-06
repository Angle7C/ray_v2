use std::sync::Arc;

use glam::{DVec3, DVec4};

use crate::pbrt_core::{
    bxdf::{reflection::LambertianReflection, BxDF},
    texture::Texture,
};

use super::{Material, BSDF};

#[derive(Debug)]
pub struct Disney {
    color: Option<Box<dyn Texture<DVec3>>>,
    metaillic: Option<Arc<dyn Texture<f64>>>,
    eta: Option<Arc<dyn Texture<f64>>>,
    roughness: Option<Arc<dyn Texture<f64>>>,
    specular: Option<Arc<dyn Texture<f64>>>,
    anisotropic_tint: Option<Arc<dyn Texture<f64>>>,
    sheen: Option<Arc<dyn Texture<f64>>>,
    clear_coat: Option<Arc<dyn Texture<f64>>>,
    clear_coat_gloss: Option<Arc<dyn Texture<f64>>>,
    spec_trans: Option<Arc<dyn Texture<f64>>>,
    scatter_distance: Option<Arc<dyn Texture<f64>>>,
    thin: bool,
    flat_ness: Option<Arc<dyn Texture<f64>>>,
    diff_trans: Option<Arc<dyn Texture<f64>>>,
    bump_map: Option<Arc<dyn Texture<f64>>>,
}
impl Disney {
    pub fn new(color: Option<Box<dyn Texture<DVec3>>>) -> Self {
        Self {
            color,
            metaillic: None,
            eta: None,
            roughness: None,
            specular: None,
            anisotropic_tint: None,
            sheen: None,
            clear_coat: None,
            clear_coat_gloss: None,
            spec_trans: None,
            scatter_distance: None,
            thin: false,
            flat_ness: None,
            diff_trans: None,
            bump_map: None,
        }
    }
}
impl Material for Disney {
    fn compute_scattering_functions(
        &self,
        suface: &mut crate::pbrt_core::tool::SurfaceInteraction,
        mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        if let Some(color) = &self.color {
            let mut bsdf = BSDF::new(&suface, 1.0);
            bsdf.bxdfs
                .push(BxDF::LambertianReflection(LambertianReflection::new(
                    color.evaluate(&suface.common),
                )));
            suface.bsdf = Some(bsdf)
        }
    }
    fn bump(
        &self,
        suface: &crate::pbrt_core::tool::SurfaceInteraction,
        texture: &dyn Texture<f64>,
    ) {
    }
}
