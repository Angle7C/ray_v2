use std::default::Default;
use std::fmt::Debug;
use std::sync::Arc;
use crate::pbrt_core::light::{LightAble, LightType};
use crate::pbrt_core::primitive::shape::ShapeAble;
use crate::pbrt_core::primitive::GeometricPrimitive;
use crate::pbrt_core::{
    camera::Camera,
    primitive::{bvh::BVH, Aggregate,Primitive},
};

use super::color::Color;
use super::{Bound, RayDiff, SurfaceInteraction};

pub struct Scene {
    pub camera: Camera,
    pub lights:Vec<Arc<dyn LightAble>>,
    pub env:Vec<Arc<dyn LightAble>>,
    pub shapes_light:Vec<Arc<dyn ShapeAble>>,
    pub shapes_env:Vec<Arc<dyn ShapeAble>>,
    bound: Bound<3>,
    accel: Box<dyn Aggregate>
}

unsafe impl Sync for Scene {}

impl Scene {
    pub fn new(primitive: Vec<GeometricPrimitive>,camera:Camera) -> Self {
        
        let mut env=Vec::new();
        let mut lights=vec![];
        let mut shapes_light=Vec::new();
        let mut shapes_env=Vec::new();

        primitive.iter()
        .for_each(|item|{
            if let Some(light)=item.get_arc_light() {
                if LightType::is_inf(light.get_type()){
                    env.push(light.clone());
                    shapes_env.push(item.shape.clone());
                }
                lights.push(light.clone());
                shapes_light.push(item.shape.clone());
            }
        });

       let bound=primitive.iter()
        .map(|primitive|primitive.world_bound())
        .fold(Bound::<3>::default(), |a,b| b.merage(a));
        assert_ne!(primitive.len(),0);

        let accel=BVH::new(primitive);
       
        Scene { camera, lights, bound,env, accel: Box::new(accel),shapes_env,shapes_light }
    }
}

impl Scene {
    pub fn sample_env_light(&self, ray: &RayDiff) -> Color {
        
        if self.env.is_empty(){
            return Color::ZERO
        }
        let mut ans = Color::default();
        for env_light in &self.env {
                ans+=env_light.le(ray,None);
        }
        ans
    }
    pub fn intersect_p(&self,ray:&RayDiff)->bool{
        if self.bound.intesect(ray){
            self.accel.intersect_p(ray)
        }else{
            false
        }
    }
    pub fn intersect(&self,ray:RayDiff)->Option<SurfaceInteraction>{
        if self.bound.intesect(&ray){
            self.accel.intersect(&ray)
        }else{
            None
        }
        
    }
}

impl Debug for Scene {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

