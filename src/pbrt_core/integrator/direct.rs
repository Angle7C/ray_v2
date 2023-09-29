use glam::Vec3;
use serde::{Serialize, Deserialize};

use crate::pbrt_core::{
    primitive::Primitive,
    sampler::Sampler,
    tool::{sence::Sence, RayDiff, color::Color},
};

use super::uniform_sample_all_light;

pub struct DirectIntegrator {
    _strategy: LightStartegy,
    _sample: Sampler,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub enum LightStartegy {
    UniformAll,
    UniformOne,
}
impl DirectIntegrator {
    pub fn new(_max_depth: u32, strategy: LightStartegy, sample: Sampler) -> Self {
        Self {
            _strategy:strategy,
            _sample:sample,
        }
    }
    pub fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> Color {
        let mut ans = Vec3::ZERO;
        let beta=Vec3::ONE;
        let n_sample=vec![1,1,1,1];
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;
        if let Some(mut item) = sence.interacect(ray) {
            if item.light.is_some() {
                ans += beta * item.le(ray);
                return ans;
            }
            item.compute_scattering(ray, mode);
            if let Some(_) = &item.bsdf {
                ans += beta * uniform_sample_all_light(&item, sence, sampler.clone(),n_sample,false);
            }
        }
        ans
    }
}
