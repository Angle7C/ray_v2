use std::fmt::Debug;

use glam::{Vec2, Vec3};
use log::info;
use rand::Rng;

use crate::pbrt_core::{
    bxdf::BxDFType,
    camera::Camera,
    light::{Light, LightAble},
    primitive::{bvh::BVH, Aggregate, GeometricePrimitive, Primitive},
    sampler::Sampler,
};

use super::{Bound, SurfaceInteraction, Visibility};
pub struct Sence {
    shape: &'static [Box<dyn Primitive>],
    pub camera: Camera,
    light: &'static [Light],
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
        // material: [Box<dyn Material>],
    ) -> Self {
        let light = light.leak();
        let primitive = primitive.leak();
        //场景集合

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

        let accel = Box::new(BVH::new(geoemtry));

        let sence = Self {
            shape: primitive,
            camera,
            bound,
            light: light,
            accel: Some(accel),
        };
        sence
    }
}
impl Sence {
    pub fn uniform_sample_one_light(
        &self,
        surface: &SurfaceInteraction,
        sampler: &mut Sampler,
        //是否有介质参与
        handle: bool,
    ) -> Vec3 {
        //随机选择一个光源
        let light_num = sampler.rand.gen_range(0..self.light.len());
        let light = &self.light[light_num];

        let mut s = Vec3::ZERO;
        for _ in 0..8 {
            //采样光源点
            let point = sampler.sample_2d_d();
            //采样吸收点
            s += sample_light(
                surface,
                point,
                light,
                self,
                sampler,
                31,
                handle,
            )
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
        Light::AreaLight(area) => area.sample_li(surface, u, &mut wi, &mut light_pdf, &mut vis),
        Light::PointLight(p) => p.sample_li(surface, u, &mut wi, &mut light_pdf, &mut vis),
    };
    let f = if let Some(bsdf) = &surface.bsdf {
        let f = bsdf.f(&surface.common.w0, &wi, flag) * wi.dot(surface.shading.n).abs();
        // scattering_pdf = bsdf.pdf(&surface.common.w0, &mut wi, flag);
        f
    } else {
        Vec3::ZERO
    };
    let ok = vis.g(sence);
    if f.abs_diff_eq(Vec3::ZERO, f32::EPSILON){
        info!("bsdf {:?}",f);
    }
    ld * ok * f / light_pdf
}
