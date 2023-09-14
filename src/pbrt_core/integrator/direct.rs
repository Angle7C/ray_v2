use glam::Vec3;

use crate::pbrt_core::{
    primitive::Primitive,
    sampler::Sampler,
    tool::{sence::Sence, RayDiff, color::Color},
};

pub struct DirectIntegrator {
    _max_depth: u32,
    _strategy: LightStartegy,
    _sample: Sampler,
}
pub enum LightStartegy {
    UniformAll,
    UniformOne,
}
impl DirectIntegrator {
    pub fn new(max_depth: u32, strategy: LightStartegy, sample: Sampler) -> Self {
        Self {
            _max_depth:max_depth,
            _strategy:strategy,
            _sample:sample,
        }
    }
    pub fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> Color {
        let mut ans = Vec3::ZERO;
        let beta=Vec3::ONE;
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;
        if let Some(mut item) = sence.interacect(ray) {
            if item.light.is_some() {
                ans += beta * item.le(ray.o.dir);
                return ans;
            }
            item.compute_scattering(ray, mode);
            if let Some(_) = &item.bsdf {
                ans += beta * sence.uniform_sample_one_light(&item, sampler, false);
            }
        }
        ans
    }
}
