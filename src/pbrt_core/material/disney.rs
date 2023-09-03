use std::sync::Arc;

use glam::DVec3;

use crate::pbrt_core::texture::Texture;

pub struct Disney{
    color:Option<Arc<dyn Texture<DVec3>>>,
    metaillic:Option<Arc<dyn Texture<f64>>>,
    eta:Option<Arc<dyn Texture<f64>>>,
    roughness:Option<Arc<dyn Texture<f64>>>,
    specular:Option<Arc<dyn Texture<f64>>>,
    anisotropic_tint:Option<Arc<dyn Texture<f64>>>,
    sheen:Option<Arc<dyn Texture<f64>>>,
    clear_coat:Option<Arc<dyn Texture<f64>>>,
    clear_coat_gloss:Option<Arc<dyn Texture<f64>>>,
    spec_trans:Option<Arc<dyn Texture<f64>>>,
    scatter_distance:Option<Arc<dyn Texture<f64>>>,
    thin:bool,
    flat_ness:Option<Arc<dyn Texture<f64>>>,
    diff_trans:Option<Arc<dyn Texture<f64>>>,
    bump_map:Option<Arc<dyn Texture<f64>>>,
}