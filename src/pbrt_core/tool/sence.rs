use std::borrow::Borrow;
use std::default::Default;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, Weak};

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
    pub env:Vec<Weak<Light>>,
    bound: Bound<3>,
    accel: Option<Box<dyn Aggregate>>,
}

unsafe impl Sync for Scene {}

impl Scene {
    pub fn new(primitive: Vec<Arc<dyn Primitive>>, camera: Camera, light: Vec<Arc<Light>>) -> Self {
        // let primitive = primitive.leak();

        //场景集合
        // let light = light.leak();
        let mut geometry = primitive.iter()
            .map(|ele| GeometricPrimitive::new(ele.clone()))
            .collect::<Vec<_>>();

        light.iter().for_each(|item|{
            let index   = item.get_index();
            if let Some(geo)= geometry.get_mut(index){
               geo.set_light(item.clone());
            };
        });
        let light = light.iter()
            .map(|item| item.clone())
            .collect::<Vec<_>>();


        let bound = geometry
            .iter()
            .map(|ele| ele.world_bound())
            .fold(Bound::<3>::default(), |a, b| a.merage(b));
        let mut env = vec![];
        light.iter().for_each(|i| {
            match i.as_ref() {
                Light::Infinite(_) => env.push(Arc::downgrade(&i)),
                _ => (),
            };
        });

        let accel = Box::new(BVH::new(geometry));
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
            //究极解引用
            {
                ans+=LightAble::le(env_light.upgrade().as_ref().unwrap().deref().deref(),ray);
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
                accel.interact(&ray)
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
