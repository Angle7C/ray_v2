use std::fmt::Debug;

use glam::DVec3;

use crate::pbrt_core::{
    camera::Camera,
    light::Light,
    primitive::{bvh::BVH, Aggregate, GeometricePrimitive, ObjectType, Primitive},
};

use super::{Bound, SurfaceInteraction};

pub struct Sence<'a> {
    primitive: &'a[Box<dyn Primitive>],
    accel: Box<dyn Aggregate>,
    bound: Bound<3>,
    light: Vec<Light>,
    pub camera: Camera,
}
impl<'a> Debug for Sence<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
unsafe impl<'a> Sync for Sence<'a>{}

impl<'a> Sence<'a>{
    pub fn new(primitive: Vec<Box<dyn Primitive>>, light: Vec<Light>, camera: Camera) -> Self {
        let  primitive= primitive.leak();
        let bound = primitive
            .iter()
            .map(|ele| ele.world_bound())
            .fold(Bound::<3>::default(), |a, b| a.merage(b));
        //场景集合
        let geoemtry = primitive
            .iter()

            .map(|ele| GeometricePrimitive::new(ele.as_ref(), ObjectType::Sence))
            .collect::<Vec<_>>();
        
        let accel: BVH<'_> = BVH::new(geoemtry);
        //添加light 
        // todo!();
        let sence=Self {
            primitive,
            accel: Box::new(accel),
            bound,
            light:light,
            camera,
        };
        sence
    }

    pub fn sample_light(&self,surface:&SurfaceInteraction)->DVec3{
        todo!()
    }
}

impl<'a> Primitive for Sence<'a>{
    fn interacect(&self,ray:super::RayDiff)->Option<super::SurfaceInteraction> {
        if(self.interacect_bound(&ray)){
            self.accel.interacect(&ray)

        }else{
            None
        }

    }
    fn world_bound(&self)->Bound<3> {
        self.bound   
    }

    fn interacect_bound(&self,ray:&super::RayDiff)->bool{
        self.world_bound().intesect(ray)
    }    
}