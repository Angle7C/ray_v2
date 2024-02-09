use std::borrow::Borrow;
use std::default::Default;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::Arc;

use crate::pbrt_core::light::{Light, LightAble};
use crate::pbrt_core::{
    camera::Camera,
    primitive::{bvh::BVH, Aggregate, GeometricPrimitive, Primitive},
};

use super::color::Color;
use super::{Bound, RayDiff};

pub struct Scene {
    pub camera: Camera,
    pub light:Vec<Arc<Light>>,
    pub env:Vec<Weak<Arc<Light>>>,
    bound: Bound<3>,
    accel: Option<Box<dyn Aggregate>>,
}

unsafe impl Sync for Scene {}

impl Scene {
    pub fn new(primitive: Vec<Arc<dyn Primitive>>, camera: Camera, light: Vec<Arc<Light>>) -> Self {
        // let primitive = primitive.leak();

        //场景集合
        // let light = light.leak();
        let geoemtry = primitive.iter()
            .map(|ele| GeometricPrimitive::new(ele.clone()))
            .collect::<Vec<_>>();
    
        let bound = geoemtry
            .iter()
            .map(|ele| ele.world_bound())
            .fold(Bound::<3>::default(), |a, b| a.merage(b));
        let mut env = vec![];
        light.iter().for_each(|i| {
            match i.as_ref() {
                Light::Infinite(_) => env.push(Rc::downgrade(&Rc::new(i.clone()))),
                _ => (),
            };
        });

        let accel = Box::new(BVH::new(geoemtry));
        Self {
            camera,
            bound,
            light,
            env,
            accel: Some(accel),
        }
    }
}

impl Scene {
    pub fn sample_env_light(&self, ray: &RayDiff) -> Color {
        if self.env.is_empty(){
            return Color::default();
        }
        let mut ans = Color::default();
        for env_light in &self.env {

            {
                ans+=LightAble::le(env_light.upgrade().as_ref().unwrap().deref(),ray);
            }
        }
        ans
    }
}

impl Debug for Scene {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Primitive for Scene {
    fn interact(&self, ray: super::RayDiff) -> Option<super::SurfaceInteraction> {
        if self.interact_bound(&ray) {
            if let Some(accel) = &self.accel {
                accel.interacect(&ray)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn hit_p(&self, ray: &super::RayDiff) -> bool {
        if self.interact_bound(ray) {
            if let Some(accel) = &self.accel {
                accel.hit_p(ray)
            } else {
                false
            }
        } else {
            false
        }
    }
    fn world_bound(&self) -> Bound<3> {
        self.bound
    }

    fn interact_bound(&self, ray: &super::RayDiff) -> bool {
        self.world_bound().intesect(ray)
    }
}
