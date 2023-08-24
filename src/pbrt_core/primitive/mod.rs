
use std::fmt::Debug;

use ::bvh::{bounding_hierarchy::BHShape, aabb::Bounded};
use glam::f32::Vec3;

use super::{tool::{ InteractionCommon, RayDiff, SurfaceInteraction, Bound}, bxdf::TransportMode};

pub mod bvh;
pub mod mesh;
pub mod shape{
    pub mod shpere;
    pub mod triangle;  
    pub enum Shape {
        
    }
}
pub trait Primitive:Debug {
    fn world_bound(&self)->Bound<3>;
    fn interacect(&self,_ray:RayDiff)->Option<SurfaceInteraction>{
        None
    }
    fn interacect_bound(&self,ray:&RayDiff)->bool{
        self.world_bound().intesect(ray)
    }
    fn compute_scattering(&self,isct:&mut SurfaceInteraction,mode:TransportMode){

    }
}
pub trait Aggregate:Sync{
    fn interacect(&self, ray: &RayDiff) -> Option<SurfaceInteraction>;
}
#[derive(Debug)]
pub enum ObjectType{
    Light,
    Sence
}
impl PartialEq for ObjectType{
    fn eq(&self, other: &Self) -> bool {
        match (self,other){
            (ObjectType::Light, ObjectType::Light) => true,
            (ObjectType::Sence, ObjectType::Sence) => true,
            _=>false
        }
    }
}
#[derive(Debug)]
pub struct GeometricePrimitive<'a>{
    shape:&'a dyn Primitive,
    mode:ObjectType,
    node_index:usize,

}
unsafe impl<'a> Sync for GeometricePrimitive<'a>{

}
unsafe impl<'a> Send for GeometricePrimitive<'a>{

}
impl<'a> GeometricePrimitive<'a>{
    pub fn new(shape:&'a dyn Primitive,mode:ObjectType)->Self{
        Self{shape,mode,node_index:0}
    }
}
impl<'a> Bounded for GeometricePrimitive<'a>{
    fn aabb(&self) -> ::bvh::aabb::AABB {
        let bound = self.shape.world_bound();
        bound.into()
    }
}
impl<'a> BHShape for GeometricePrimitive<'a>{
    fn bh_node_index(&self) -> usize {
        self.node_index
    }
    fn set_bh_node_index(&mut self, i: usize) {
        self.node_index=i
    }
}
impl<'a> Primitive for GeometricePrimitive<'a>
{
    fn compute_scattering(&self,isct:&mut SurfaceInteraction,mode:TransportMode) {
        self.shape.compute_scattering(isct, mode)
    }
    fn interacect(&self,ray:RayDiff)->Option<SurfaceInteraction> {
        let mut iter = self.shape.interacect(ray);
        if let Some(ref mut i) =&mut iter  {
            i.common.is_light=self.mode==ObjectType::Light;
        };
        iter
    }
    fn interacect_bound(&self,ray:&RayDiff)->bool {
        self.shape.interacect_bound(ray)        
    }
    fn world_bound(&self)->Bound<3> {
        self.shape.world_bound()
    }
}