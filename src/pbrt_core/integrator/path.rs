use std::{process::exit, time::{Duration, SystemTime}, io::stdout, ops::Sub};

use glam::Vec3;
use log::info;


use crate::pbrt_core::{
    bxdf::BxDFType,
    primitive::Primitive,
    sampler::Sampler,
    tool::{color::Color, sence::Sence, RayDiff},
};

use super::{unifrom_sample_one_light, IntegratorAble};

//路径追踪积分器
pub struct PathIntegrator {
    q: f32,
    max_path: usize,
}

impl Default for PathIntegrator {
    fn default() -> Self {
        Self {
            q: 0.9,
            max_path: 10,
        }
    }
}

impl IntegratorAble for PathIntegrator {
    fn is_next(&self, dept: &mut usize) -> Option<f32> {
        *dept += 1;
        if *dept > self.max_path {
            // let p: f32 = rand::random();
            // if p > self.q {
            //     None
            // } else {
            //     Some(1.0 -p)
            // }
            None
        } else {
            Some(1.0)
        }
    }
    fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> Color {
        let mut ans = Color::ZERO;
        let mut dept = 0;
        let mut beta: Vec3 = Vec3::ONE;
        let mut ray = ray.clone();
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;   
        while let Some(p) = self.is_next(&mut dept) {
            if let Some(mut item) = sence.interacect(ray) {
                if item.light.is_some() {
                    ans += beta * item.le(ray);
                    // ans+=Vec3::X;
                    return ans;
                }
                item.compute_scattering(ray, mode);
                // ans+=beta*sence.sample_env_light(&ray);
             
                if let Some(bsdf) = &item.bsdf {
                    //场景光源采样
                    ans += beta * unifrom_sample_one_light(&item, sence, sampler.clone(), false);
                    //BRDF 采样生成光线
                    let w_out = -ray.o.dir;
                    let mut w_in = Vec3::default();
                    let mut pdf = 0.0;
                    let mut samped_type: u32 = 0;
                    let f = bsdf.sample_f(
                        &w_out,
                        &mut w_in,
                        sampler.sample_2d_d(),
                        &mut pdf,
                        BxDFType::All.into(),
                        &mut samped_type,
                    ) * w_in.dot(item.shading.n).abs()
                        / pdf;
                    beta *= f;
                    ray = item.spawn_ray(&w_in);
                    // stack.push((ans,f,beta,ray));
                    
                }
            } else {
                ans+=beta*sence.sample_env_light(&ray);
                //环境光采样
                break;
            }
            beta = beta / p;
        }
        ans
    }
}

impl PathIntegrator {
    pub fn new(q: f32, max_path: usize) -> Self {
        Self { q, max_path }
    }
}
