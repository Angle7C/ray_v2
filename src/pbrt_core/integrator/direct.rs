use glam::Vec3;
use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    light::LightAble,
    primitive::Primitive,
    sampler::Sampler,
    tool::{color::Color, sence::Sence, RayDiff},
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
            _strategy: strategy,
            _sample: sample,
        }
    }
    pub fn fi(
        &self,
        ray: RayDiff,
        sence: &Sence,
        sampler: &mut Sampler,
        #[cfg(debug_assertions)] i: &mut i32,
    ) -> Color {
        let mut ans = Vec3::ZERO;
        let beta = Vec3::ONE;
        let mut n_sample = vec![];
        for i in sence.light {
            n_sample.push(i.get_n_sample());
        }
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;
        if let Some(mut item) = sence.interacect(ray) {
            if item.light.is_some() {
                ans += beta * item.le(ray);
                return ans;
            }
            item.compute_scattering(ray, mode);
            if let Some(_bsdf) = &item.bsdf {
                // return (item.common.normal+Vec3::ONE)/2.0;
                ans +=
                    beta * uniform_sample_all_light(&item, sence, sampler.clone(), n_sample, false);
                #[cfg(debug_assertions)]
                {
                    *i += 1;
                }
            }
        }
        #[cfg(debug_assertions)]
        if *i>0 {
            warn!("sub color {}",ans);
        }
        ans
    }
}
