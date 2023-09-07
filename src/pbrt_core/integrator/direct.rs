use glam::DVec3;

use crate::pbrt_core::{
    bxdf::{BxDFType, TransportMode},
    primitive::Primitive,
    sampler::Sampler,
    tool::{sence::Sence, Ray, RayDiff},
};

pub struct DirectIntegrator {
    max_depth: u32,
    strategy: LightStartegy,
    sample: Sampler,
}
pub enum LightStartegy {
    UniformAll,
    UniformOne,
}
impl DirectIntegrator {
    pub fn new(max_depth: u32, strategy: LightStartegy, sample: Sampler) -> Self {
        Self {
            max_depth,
            strategy,
            sample,
        }
    }
    pub fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> DVec3 {
        let mut ans = DVec3::ZERO;
        let mut dept = 0;
        let mut beta: DVec3 = DVec3::ONE;
        let mut ray = ray.clone();
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;
        if let Some(mut item) = sence.interacect(ray) {
            if item.light.is_some() {
                ans += beta * item.le(ray.o.dir);
                return ans;
            }
            item.compute_scattering(ray, mode);
            if let Some(bsdf) = &item.bsdf {
                //场景光源采样
                ans += beta * sence.uniform_sample_one_light(&item, sampler, false);
                //BRDF 采样生成光线
                //     let w_out = -ray.o.dir;
                //     let mut w_in = DVec3::default();
                //     let mut pdf = 0.0;
                //     let mut flags: u32 = BxDFType::All as u32;
                //     let f = bsdf.sample_f(&w_out, &mut w_in, sampler.sample_2d_d(), &mut pdf, flags);
                //     beta *= f * w_in.dot(item.shading.n).abs();
                //     ray = item.spawn_ray(&w_in);
                // }
            }
        }
        ans
    }
}
