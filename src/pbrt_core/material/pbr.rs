use std::sync::Arc;

use glam::Vec3;

use crate::pbrt_core::{
    bxdf::{
        frensnel::{DisneyFrenel, Fresnel},
        microfacet_distribution::{roughness_to_alpha, TrowbridgeReitzDistribution},
        pbr::{PbrDiff, PbrReflection},
        BxDF,
    },
    texture::Texture,
    tool::SurfaceInteraction,
};

use super::{Material, BSDF};
#[derive(Debug)]
pub struct PbrMaterial {
    //基本颜色
    base_color: Option<Arc<dyn Texture>>,
    //金属度
    metailc: Option<Arc<dyn Texture>>,
    //粗糙度
    roughness: Option<Arc<dyn Texture>>,
    //亮度
    _emissive: Option<Arc<dyn Texture>>,
    //遮挡贴图
    _occlusion: Option<Arc<dyn Texture>>,
    //法线贴图
    _normal: Option<Arc<dyn Texture>>,
}
impl PbrMaterial {
    pub fn new(
        base_color: Option<Arc<dyn Texture>>,
        //金属度
        metailc: Option<Arc<dyn Texture>>,
        //粗糙度
        roughness: Option<Arc<dyn Texture>>,
        //亮度
        emissive: Option<Arc<dyn Texture>>,
        //遮挡贴图
        occlusion: Option<Arc<dyn Texture>>,
        //法线贴图
        normal: Option<Arc<dyn Texture>>,
    ) -> Self {
        Self {
            base_color,
            metailc,
            roughness,
            _emissive:emissive,
            _occlusion:occlusion,
            _normal:normal,
        }
    }
}
impl Material for PbrMaterial {
    fn bump(&self, _suface: &SurfaceInteraction, _texture: &dyn Texture) {}
    fn compute_scattering_functions(
        &self,
        suface: &mut crate::pbrt_core::tool::SurfaceInteraction,
        _mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        let r = self
            .base_color
            .as_ref()
            .unwrap()
            .evaluate(&suface.common)
            .clamp(Vec3::ZERO, Vec3::splat(f32::INFINITY));
        let metallic = if let Some(ref metallic) = self.metailc {
            metallic.evaluate(&suface.common)
        } else {
            Vec3::ZERO
        };
        let r0 = metallic * r + (Vec3::ONE - metallic) * Vec3::splat(0.04);
        let roughness = if let Some(ref roughness) = self.roughness {
            roughness.evaluate(&suface.common)
        } else {
            Vec3::splat(0.5)
        };
        let roughness = roughness_to_alpha(roughness.y);
        suface.bsdf = Some(BSDF::new(&suface, 1.0));
        if let Some(bsdf) = &mut suface.bsdf {
            if r != Vec3::ZERO {
                bsdf.bxdfs.push(BxDF::PbrDiff(PbrDiff::new(r)));
                bsdf.bxdfs.push(BxDF::PbrReflection(PbrReflection::new(
                    r,
                    Box::new(TrowbridgeReitzDistribution::new(
                        roughness, roughness, false,
                    )),
                    Fresnel::Disney(DisneyFrenel::new(r0, metallic.y, 1.0)),
                )))
            }
        }
    }
}
