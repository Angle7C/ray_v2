use std::default::Default;
use std::fmt::Debug;





use crate::pbrt_core::{
    camera::Camera,
    primitive::{bvh::BVH, Aggregate, GeometricePrimitive, Primitive},
};
use crate::pbrt_core::light::Light;

use super::{Bound};

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

