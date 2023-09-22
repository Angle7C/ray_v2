use std::default::Default;
use std::fmt::Debug;

use glam::{Vec2, Vec3};
use log::info;
use rand::Rng;

use crate::pbrt_core::{
    camera::Camera,
    light::{LightAble},
    primitive::{bvh::BVH, Aggregate, GeometricePrimitive, Primitive},
    sampler::Sampler,
};
use crate::pbrt_core::light::Light;

use super::{Bound, SurfaceInteraction, Visibility};

pub struct Sence {
    shape: &'static [Box<dyn Primitive>],
    pub camera: Camera,
    pub light: &'static [Light],
    env: Vec<&'static Light>,
    bound: Bound<3>,
    // material: Vec<Box<dyn Material>>,
    accel: Option<Box<dyn Aggregate>>,
}

unsafe impl Sync for Sence {}

impl Sence {
    pub fn new(
        primitive: Vec<Box<dyn Primitive>>,
        camera: Camera,
        light: Vec<Light>,
    ) -> Self {
        let primitive = primitive.leak();

        //场景集合
        let light = light.leak();
        let mut geoemtry = primitive
            .iter()
            .map(|ele| GeometricePrimitive::new(ele.as_ref()))
            .collect::<Vec<_>>();
        let mut geoemtry_light = light
            .iter()
            .map(|item| GeometricePrimitive::new(item))
            .collect::<Vec<_>>();
        geoemtry.append(&mut geoemtry_light);
        let bound = geoemtry
            .iter()
            .map(|ele| ele.world_bound())
            .fold(Bound::<3>::default(), |a, b| a.merage(b));
        let mut env: Vec<&Light> = vec![];
        let mut t = vec![];
        light.iter().for_each(|i| {
            if match i {
                Light::Infinite( _) => true,
                _ => false,
            } {
                env.push(i);
            } else {
                t.push(i);
            }
        });

        let accel = Box::new(BVH::new(geoemtry));
        // let x = t.leak();
        let sence = Self {
            shape: primitive,
            camera,
            bound,
            light: light,
            env: env,
            accel: Some(accel),
        };
        sence
    }
}

impl Sence {
    pub fn has_env(&self) -> bool {
        !self.env.is_empty()
    }
    pub fn get_env_light(
        &self,
        surface: &SurfaceInteraction,
        wi: Vec3,
        u: Vec2,
        flag: u32,
        _handle: bool,
    ) -> Vec3 {
        //射出
        let mut vis = Visibility::default();
        let mut ans = Vec3::ZERO;
        let mut pdf = 0.0;
        let mut wi = wi;
        for light in &self.env {
            let f = if let Some(bsdf) = &surface.bsdf {
                let f = bsdf.sample_f(&surface.common.w0, &mut wi, u, &mut pdf, 31) * wi.dot(surface.shading.n).abs();
                f
            } else {
                Vec3::ZERO
            };
            let ld = match light {
                Light::Infinite(inf) => inf.sample_li(surface,u,&mut wi,&mut light_pdf,&mut vis),
                _ => todo!(),
            };

            ans += ld * f *vis.g_inf(self);
        };
        ans
    }
    pub fn get_hit_env(&self, wi: Vec3) -> Vec3 {
        let mut ans = Vec3::ZERO;
        for light in &self.env {
            let ld = match light {
                Light::Infinite(inf) => inf.le(wi),
                _ => todo!(),
            };
            ans += ld;
        }
        ans
    }

    pub fn uniform_sample_one_light(
        &self,
        surface: &SurfaceInteraction,
        sampler: &mut Sampler,
        //是否有介质参与
        handle: bool,
    ) -> Vec3 {
        if self.light.len() == 0 {
            return Vec3::ZERO;
        }
        //随机选择一个光源
        let light_num = sampler.rand.gen_range(0..self.light.len());
        let light = &self.light[light_num];

        let mut s = Vec3::ZERO;
        for _ in 0..8 {
            //采样光源点
            let point = sampler.sample_2d_d();
            //采样吸收点
            s += sample_light(surface, point, light, self, sampler, 31, handle)
        }
        s / 8.0
    }
    pub fn uniform_sample_all_light(
        &self,
        _surface: &SurfaceInteraction,
        _sampler: &mut Sampler,
        //是否有介质参与
        _handle: bool,
    ) -> Vec3 {
        unimplemented!()
    }
}

impl Debug for Sence {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Primitive for Sence {
    fn interacect(&self, ray: super::RayDiff) -> Option<super::SurfaceInteraction> {
        if self.interacect_bound(&ray) {
            if let Some(accel) = &self.accel {
                accel.interacect(&ray)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn world_bound(&self) -> Bound<3> {
        self.bound
    }

    fn interacect_bound(&self, ray: &super::RayDiff) -> bool {
        self.world_bound().intesect(ray)
    }
}

pub fn sample_light(
    surface: &SurfaceInteraction,
    u: Vec2,
    light: &Light,
    sence: &Sence,
    _sampler: &mut Sampler,
    flag: u32,
    _handle: bool,
) -> Vec3 {
    let mut light_pdf = 0.0;
    // let mut scattering_pdf = 0.0;
    let mut vis = Visibility::default();
    //射出
    let mut wi = Default::default();

    //采样值
    let ld = match light {
        Light::AreaLight(area) => area.sample_li(surface, u, &mut wi, &mut light_pdf, &mut vis)*vis.g(sence),
        Light::PointLight(p) => p.sample_li(surface, u, &mut wi, &mut light_pdf, &mut vis)*vis.g(sence),
        Light::Infinite(inf) =>inf.sample_li(surface, u, &mut wi, &mut light_pdf, &mut vis)*vis.g(sence),
    };
    let f = if let Some(bsdf) = &surface.bsdf {
        let f = bsdf.f(&surface.common.w0, &wi, flag) * wi.dot(surface.shading.n).abs();
        // scattering_pdf = bsdf.pdf(&surface.common.w0, &mut wi, flag);
        f
    } else {
        Vec3::ZERO
    };
    ld * f / light_pdf
}
