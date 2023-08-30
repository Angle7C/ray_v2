use std::{borrow::BorrowMut, fmt::Debug};

use glam::{DVec2, DVec3};
use rand::Rng;

use crate::pbrt_core::{
    bxdf::BxDFType,
    camera::Camera,
    light::{Light, LightAble},
    primitive::{self, bvh::BVH, Aggregate, GeometricePrimitive, ObjectType, Primitive},
    sampler::Sampler,
};

use super::{Bound, InteractionCommon, SurfaceInteraction, Visibility, Ray, RayDiff};

pub struct Sence<'a> {
    primitive: &'a [Box<dyn Primitive>],
    accel: Option<Box<dyn Aggregate>>,
    bound: Bound<3>,
    light: &'a [Light],
    pub camera: Camera,
}
impl<'a> Debug for Sence<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
unsafe impl<'a> Sync for Sence<'a> {}

impl<'a> Sence<'a> {
    pub fn new(primitive: Vec<Box<dyn Primitive>>, light: Vec<Light>, camera: Camera) -> Self {
        let bound = primitive
            .iter()
            .map(|ele| ele.world_bound())
            .fold(Bound::<3>::default(), |a, b| a.merage(b));

        let primitive = primitive.leak();
        let light = light.leak();
        //场景集合
        let mut geoemtry = primitive
            .iter()
            .map(|ele| GeometricePrimitive::new(ele.as_ref()))
            .collect::<Vec<_>>();
        let mut geoemtry_light = light
            .iter()
            .map(|item| GeometricePrimitive::new(item.get_shape()))
            .collect::<Vec<_>>();
        geoemtry.append(&mut geoemtry_light);
        let accel: BVH<'_> = BVH::new(geoemtry);
        let mut sence = Self {
            primitive: primitive,
            accel: Some(Box::new(accel)),
            bound,
            light: light,
            camera,
        };
        sence
    }
    pub fn uniform_sample_one_light(
        &self,
        surface: &SurfaceInteraction,
        sampler: &mut Sampler,
        //是否有介质参与
        handle:bool
    ) -> DVec3 {
        let light_num = sampler.rand.gen_range(0..self.light.len());
        let light = &self.light[light_num];
        let point = sampler.sample_2d_d();
        let scattering = sampler.sample_2d_d();
        let mut s=DVec3::ZERO;
        for _ in 0..8{
            s+=sample_light(
                surface,
                point,
                light,
                self,
                sampler,
                BxDFType::All.into(),
                handle,
            )
        }
        s/8.0
    }
    pub fn uniform_sample_all_light( 
        &self,
        surface: &SurfaceInteraction,
        sampler: &mut Sampler,
        //是否有介质参与
        handle:bool)->DVec3{
           unimplemented!()   
        }
}

impl<'a> Primitive for Sence<'a> {
    fn interacect(&self, ray: super::RayDiff) -> Option<super::SurfaceInteraction> {
        if (self.interacect_bound(&ray)) {
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
    u: DVec2,
    light: &Light,
    sence: &Sence,
    sampler: &mut Sampler,
    flag: u32,
    handle:bool
) -> DVec3 {
    let mut ld = DVec3::ZERO;
    let mut light_pdf = 0.0;
    let mut scattering_pdf = 0.0;
    let mut vis = Visibility::default();
    let mut inter = SurfaceInteraction::default();
    let mut wi = Default::default();
    let mut ld = match light {
        Light::AreaLight(area) => area.sample_li(surface, u, &mut wi, &mut light_pdf, &mut vis),
    };
    let ok = vis.g(sence);
    ld*ok
}
fn power_heuristic(nf:u32,f_pdf:f64,ng:u32,g_pdf:f64)->f64{
    let f=nf as f64 *f_pdf;
    let g=ng as f64*g_pdf;
    return (f*f)/(f*f+g*g)
}