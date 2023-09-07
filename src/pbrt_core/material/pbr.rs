use std::sync::Arc;

use glam::DVec3;

use crate::pbrt_core::{texture::Texture, bxdf::{reflection::{LambertianReflection, OrenNayar}, BxDF}, tool::SurfaceInteraction};

use super::{Material, BSDF};
#[derive(Debug)]
pub struct PbrMaterial {
    //基本颜色
    base_color: Option<Arc<dyn Texture<DVec3>>>,
    //金属度
    metailc: Option<Arc<dyn Texture<DVec3>>>,
    //粗糙度
    roughness: Option<Arc<dyn Texture<DVec3>>>,
    //亮度
    emissive: Option<Arc<dyn Texture<DVec3>>>,
    //遮挡贴图
    occlusion: Option<Arc<dyn Texture<DVec3>>>,
    //法线贴图
    normal: Option<Arc<dyn Texture<DVec3>>>,
}
impl PbrMaterial {
    pub fn new(
        base_color: Option<Arc<dyn Texture<DVec3>>>,
        //金属度
        metailc: Option<Arc<dyn Texture<DVec3>>>,
        //粗糙度
        roughness: Option<Arc<dyn Texture<DVec3>>>,
        //亮度
        emissive: Option<Arc<dyn Texture<DVec3>>>,
        //遮挡贴图
        occlusion: Option<Arc<dyn Texture<DVec3>>>,
        //法线贴图
        normal: Option<Arc<dyn Texture<DVec3>>>,
    ) -> Self {
        Self {
            base_color,
            metailc,
            roughness,
            emissive,
            occlusion,
            normal,
        }
    }
}
impl Material for PbrMaterial {
    fn bump(
        &self,
        suface: & SurfaceInteraction,
        texture: &dyn Texture<f64>,
    ) {

    }
    fn compute_scattering_functions(
        &self,
        suface: &mut crate::pbrt_core::tool::SurfaceInteraction,
        mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        let r = self
        .base_color
        .as_ref()
        .unwrap()
        .evaluate(&suface.common)
        .clamp(DVec3::ZERO, DVec3::splat(f64::INFINITY));
    suface.bsdf = Some(BSDF::new(&suface, 1.0));
    if let Some(bsdf) = &mut suface.bsdf {
        if r != DVec3::ZERO {
            bsdf.bxdfs
                .push(BxDF::OrenNayar(OrenNayar::new(r,0.0)))
        }
    }
    }
}
