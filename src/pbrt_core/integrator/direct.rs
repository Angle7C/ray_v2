use glam::Vec3;

use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    light::LightAble,
    primitive::Primitive,
    sampler::Sampler,
    tool::{color::Color, sence::Scene, RayDiff},
};

use super::uniform_sample_all_light;

pub struct DirectIntegrator {
    _strategy: LightStrategy,
    _sample: Sampler,
}
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub enum LightStrategy {
    UniformAll,
    UniformOne,
}
impl DirectIntegrator {
    pub fn new(_max_depth:usize, strategy: LightStrategy, sample: Sampler) -> Self {
        Self {
            _strategy: strategy,
            _sample: sample,
        }
    }
    pub fn fi(
        &self,
        ray: RayDiff,
        sence: &Scene,
        sampler: &mut Sampler,
        #[cfg(debug_assertions)] _i: &mut i32,
    ) -> Color {
        let mut ans = Color::ZERO;
        let beta = Vec3::ONE;
        let mut n_sample = vec![];
        for i in &sence.lights {
            n_sample.push(i.get_samples());
        }
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;
        if let Some(mut item) = sence.intersect(ray) {
            if item.light.is_some() {
                ans += item.le(ray)*beta;
                return ans;
            }
            item.compute_scattering(ray, mode);
            if let Some(_bsdf) = &item.bsdf {
                // return (item.common.normal+Vec3::ONE)/2.0;
                ans +=  uniform_sample_all_light(&item, sence, sampler.clone(),n_sample,false)*beta;
                // ans+=beta *get_light(&item,sampler.sample_2d(),sence,sampler.clone(),false,false);
            }
        }
        ans
    }
}
