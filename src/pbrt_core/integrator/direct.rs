use glam::Vec3;
use serde::{Serialize, Deserialize};

use crate::pbrt_core::{
    primitive::Primitive,
    sampler::Sampler,
    tool::{sence::Sence, RayDiff, color::Color}, light::LightAble,
};

use super::{uniform_sample_all_light};

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
        let mut n_sample=vec![];
        for i in sence.light{
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
                ans += beta * uniform_sample_all_light(&item, sence, sampler.clone(),n_sample,false);
                // ans+=beta *get_light(&item,sampler.sample_2d(),sence,sampler.clone(),false,false);
            }
        }else{
        }
        ans
    }
}
