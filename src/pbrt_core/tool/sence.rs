use std::fmt::Debug;

use glam::{DVec3, DVec2};
use rand::Rng;

use crate::pbrt_core::{
    camera::Camera,
    light::{Light, LightAble},
    primitive::{bvh::BVH, Aggregate, GeometricePrimitive, ObjectType, Primitive}, sampler::Sampler, bxdf::BxDFType,
};

use super::{Bound, SurfaceInteraction, Visibility, InteractionCommon};

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
            .map(|ele| GeometricePrimitive::new(ele.as_ref(),None))
            .collect::<Vec<_>>();
        light.iter().map(|item|
            GeometricePrimitive::new(item.get_shape(),Some(item))).collect::<Vec<_>>();

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
    pub fn uniform_sample_one_light(&self,surface:&SurfaceInteraction,sampler:&mut Sampler)->DVec3{
        let light_num = sampler.rand.gen_range(0..self.light.len());
        let light=&self.light[light_num];
        let sample_point=sampler.sample_2d_d();
        let sample_scattering=sampler.sample_2d_d();
        sample_light(surface,sample_point,light,self,sampler,BxDFType::All.into())
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
pub fn sample_light(surface:&SurfaceInteraction,u:DVec2,light:&Light,sence:&Sence,sampler:&mut Sampler,flag:u32)->DVec3{
    let mut ld=DVec3::ZERO;
    let mut light_pdf=0.0;
    let mut scattering_pdf=0.0;
    let mut vis=Visibility::default();
    let mut inter=SurfaceInteraction::default();
    let mut wi=Default::default();
    let li= match light{
        Light::PointLight(p)=>{
            p.sample_f(surface, u, &mut wi, &mut light_pdf,&mut vis) 
        }
        Light::AreaLight(area) =>area.sample_f(surface, u, &mut wi, &mut light_pdf,&mut vis),
    };
    let ok = vis.hit(sence);
    li*ok
}