use glam::Vec3;
use log::info;

use crate::pbrt_core::{
    bxdf::BxDFType,
    sampler::Sampler,
    tool::{color::Color, sence::Scene, RayDiff},
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
impl PathIntegrator{
    fn is_next(&self, dept: &mut usize) -> Option<f32> {
        *dept += 1;
        if *dept > self.max_path {
            let p = rand::random::<f32>();
            if p > self.q {
                None
            } else {
                Some(p)
            }
        } else {
            Some(1.0)
        }
    }
}

impl IntegratorAble for PathIntegrator {

    //光追算法
    fn fi(
        &self,
        ray: RayDiff,
        scene: &Scene,
        sampler: &mut Sampler,
        #[cfg(debug_assertions)] i: &mut i32,
    ) -> Color {
        let mut lte = Color::ZERO;
        let mut dept = 0;
        let mut beta: Vec3 = Vec3::ONE;
        let mut ray = ray;
        let mode = crate::pbrt_core::bxdf::TransportMode::Radiance;
        //蒙特卡洛
        while let Some(p) = self.is_next(&mut dept) {
            //光线求交
            if let Some(mut item) = scene.intersect(ray) {
                //击中光源，立即返回
                if item.light.is_some() {
                    lte +=  item.le(ray)*beta;
                    return lte;
                }
                //计算该点的材质
                item.compute_scattering(ray, mode);
                //计算BSDF
                if let Some(bsdf) = &item.bsdf {
                    //场景光源采样
                    lte +=
                         unifrom_sample_one_light(&item, scene, sampler.clone(), false)*beta / p;
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
                    ) * w_in.dot(item.common.shading.n).abs()
                        / pdf;
                    if f.is_nan() || f.abs_diff_eq(Vec3::ZERO, f32::EPSILON) {
                        break;
                    }
                    beta *= f;
                    ray = item.spawn_ray(&w_in);
                    #[cfg(debug_assertions)]
                    {
                        *i += 1;
                    }
                }
            } else {
                lte +=  scene.sample_env_light(&ray)*beta;
                //环境光采样
                break;
            }
            beta = beta / p;
        }
        lte
    }
}

impl PathIntegrator {
    pub fn new(q: f32, max_path: usize) -> Self {
        Self { q, max_path }
    }
}
