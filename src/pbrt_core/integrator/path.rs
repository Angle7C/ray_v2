use glam::{f64::DVec3, u32::UVec2};
use image::{Rgb, RgbImage};
use log::info;

use std::{
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::pbrt_core::{
    bxdf::BxDFType,
    camera::{Camera, CameraSample},
    integrator::to_color,
    material::BSDF,
    primitive::Primitive,
    sampler::Sampler,
    tool::{sence::Sence, Film, Ray, RayDiff},
};

use super::IntegratorAble;

//路径追踪积分器
pub struct PathIntegrator {
    q: f64,
    max_path: usize,
    sampler: Sampler,
    size: UVec2,
}
impl Default for PathIntegrator {
    fn default() -> Self {
        Self {
            q: 0.9,
            max_path: 10,
            sampler: Sampler::default(),
            size: UVec2::new(512, 512),
        }
    }
}
impl IntegratorAble for PathIntegrator {
    fn is_next(&self, dept: &mut usize) -> Option<f64> {
        *dept+=1;
        if *dept > self.max_path {
            let p:f64 = rand::random();
            if p > self.q {
                None
            } else {
                Some(1.0-self.q)
            }
        } else {
            Some(1.0)
        }
    }
    fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> DVec3 {
        let mut ans = DVec3::ZERO;
        let mut dept = 0;
        let mut beta: DVec3 = DVec3::ONE;
        let mut ray = ray.clone();
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;
        while let Some(p) = self.is_next(&mut dept) {
            if let Some(mut item) = sence.interacect(ray) {
                // return item.common.normal.abs();
                ans += beta * item.le(-ray.o.dir);
                item.compute_scattering(ray, mode);
                if let Some(bsdf) = &item.bsdf {
                    //场景光源采样
                    ans += beta * sence.uniform_sample_one_light(&item, sampler, false);
                    //BRDF 采样生成光线
                    let w_out = -ray.o.dir;
                    let mut w_in = DVec3::default();
                    let mut pdf = 0.0;
                    let mut flags: u32 = BxDFType::All as u32;
                    let f =
                        bsdf.sample_f(&w_out, &mut w_in, sampler.sample_2d_d(), &mut pdf, flags);
                    beta *= f * w_in.dot(item.shading.n).abs();
                    ray = item.spawn_ray(&w_in);
                }
                beta / p;
                // return ans;
            } else {
                return ans;
            }
        }
        ans
    }
}
impl PathIntegrator {
    pub fn new(q: f64, max_path: usize, sampler: Sampler, size: UVec2) -> Self {
        Self {
            q,
            max_path,
            sampler,
            size,
        }
    }
}
