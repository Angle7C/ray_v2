// use std::sync::Arc;

// use glam::{Vec3, Vec4};

// use crate::pbrt_core::{
//     bxdf::{reflection::LambertianReflection, BxDF},
//     texture::Texture,
// };

// use super::{Material, BSDF};

// #[derive(Debug)]
// pub struct Disney {
//     color: Option<Box<dyn Texture<Vec3>>>,
//     metaillic: Option<Arc<dyn Texture<f32>>>,
//     eta: Option<Arc<dyn Texture<f32>>>,
//     roughness: Option<Arc<dyn Texture<f32>>>,
//     specular: Option<Arc<dyn Texture<f32>>>,
//     anisotropic_tint: Option<Arc<dyn Texture<f32>>>,
//     sheen: Option<Arc<dyn Texture<f32>>>,
//     clear_coat: Option<Arc<dyn Texture<f32>>>,
//     clear_coat_gloss: Option<Arc<dyn Texture<f32>>>,
//     spec_trans: Option<Arc<dyn Texture<f32>>>,
//     scatter_distance: Option<Arc<dyn Texture<f32>>>,
//     thin: bool,
//     flat_ness: Option<Arc<dyn Texture<f32>>>,
//     diff_trans: Option<Arc<dyn Texture<f32>>>,
//     bump_map: Option<Arc<dyn Texture<f32>>>,
// }
// impl Disney {
//     pub fn new(color: Option<Box<dyn Texture<Vec3>>>) -> Self {
//         Self {
//             color,
//             metaillic: None,
//             eta: None,
//             roughness: None,
//             specular: None,
//             anisotropic_tint: None,
//             sheen: None,
//             clear_coat: None,
//             clear_coat_gloss: None,
//             spec_trans: None,
//             scatter_distance: None,
//             thin: false,
//             flat_ness: None,
//             diff_trans: None,
//             bump_map: None,
//         }
//     }
// }
// impl Material for Disney {
//     fn compute_scattering_functions(
//         &self,
//         suface: &mut crate::pbrt_core::tool::SurfaceInteraction,
//         mode: crate::pbrt_core::bxdf::TransportMode,
//     ) {
//         let r = self
//             .color
//             .as_ref()
//             .unwrap()
//             .evaluate(&suface.common)
//             .clamp(Vec3::ZERO, Vec3::splat(f32::INFINITY));
//         suface.bsdf = Some(BSDF::new(&suface, 1.0));
        
//         if let Some(bsdf) = &mut suface.bsdf {
//             if r != Vec3::ZERO {
//                 bsdf.bxdfs
//                     .push(BxDF::LambertianReflection(LambertianReflection::new(r)))
//             }
//         }
//     }
//     fn bump(
//         &self,
//         suface: &crate::pbrt_core::tool::SurfaceInteraction,
//         texture: &dyn Texture<f32>,
//     ) {
//     }
// }
