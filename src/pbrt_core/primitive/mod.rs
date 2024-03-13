use std::{fmt::Debug, sync::Arc};
use std::ops::Deref;

use ::bvh::{aabb::Bounded, bounding_hierarchy::BHShape};
use crate::pbrt_core::light::Light;

use self::shape::ShapeAble;

use super::bssdf::MediumInterface;
use super::material::Material;
use super::{
    bxdf::TransportMode,
    light::LightAble,
    tool::{Bound, RayDiff, SurfaceInteraction},
};

pub mod bvh;
pub mod mesh;

pub mod shape;
pub trait Primitive:BHShape+Bounded {

    //世界包围盒
    fn world_bound(&self) -> Bound<3>;
    //表面求交
    fn intersect(&self, _ray: RayDiff) -> Option<SurfaceInteraction>;
    //是否有交点
    fn intersect_p(&self, ray: &RayDiff) -> bool;
    //获取面光源
    fn get_light(&self)->Option<&dyn LightAble>{None}
    //获取图元面积
    fn get_area(&self) -> f32;
    //获取材质
    fn get_material(&self)->Option<&dyn Material>;
    //材质计算
    fn compute_scattering(&self, suface: &mut SurfaceInteraction, mode: TransportMode){
        if let Some(material)=self.get_material(){
            material.compute_scattering_functions(suface, mode)
        }
    }

    fn get_shape(&self)->Option<&dyn ShapeAble>;

    fn get_arc_light(&self)->Option<Arc<dyn LightAble>>{
        None
    }
}


pub trait Aggregate: Sync {
    fn intersect(&self, ray: &RayDiff) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, ray: &RayDiff) -> bool;
}
#[derive(Debug)]
pub enum ObjectType {
    Light,
    Scene,
}
impl PartialEq for ObjectType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ObjectType::Light, ObjectType::Light) => true,
            (ObjectType::Scene, ObjectType::Scene) => true,
            _ => false,
        }
    }
}

pub struct GeometricPrimitive {
    //几何图元
   pub shape: Arc<dyn ShapeAble>,
    //材质
    material:Option<Arc<dyn Material>>,
    //光源
    light: Option<Arc<dyn LightAble>>,
    //介质
    medium:Option<Arc<MediumInterface>>,
    node_index:usize
}
unsafe impl Sync for GeometricPrimitive {}
unsafe impl Send for GeometricPrimitive {}
impl GeometricPrimitive {
    pub fn new(  //几何图元
        shape: Arc<dyn ShapeAble>,
        //材质
        material:Option<Arc<dyn Material>>,
        //光源
        light: Option<Arc<dyn LightAble>>,
        //介质
        medium:Option<Arc<MediumInterface>>) -> Self {
        Self {
            shape,
            material,
            light,
            medium,
            node_index:0
        }
    }
    pub fn set_light(&mut self,light: Arc<Light>){
        self.light=Some(light);
    }
}
impl Primitive for GeometricPrimitive{
    fn get_area(&self) -> f32 {
        self.shape.area()
    }
    fn get_light(&self)->Option<&dyn LightAble> {
         if let Some(ref light)= self.light {
             Some( light.deref() as &dyn LightAble)
         }else{
            None
         }
        
    }
    fn get_arc_light(&self)->Option<Arc<dyn LightAble>> {
        if let Some(light) = &self.light {
            Some(light.clone())
        }else {
            None
        }
    }
    fn get_material(&self)->Option<& dyn Material> {
        if let Some(ref material)= self.material {
            Some( material.deref() as &dyn Material)
        }else{
           None
        }
    }
    fn intersect(&self, ray: RayDiff) -> Option<SurfaceInteraction> {
        if let Some(common) = self.shape.intersect(ray){
            let mut surface = SurfaceInteraction::default();
            surface.common=common;
            surface.light=self.get_light();
            surface.primitive=Some(self);
            surface.shape=Some(self.shape.as_ref());
            Some(surface)
        }else{
            None
        }
        
    }
    fn intersect_p(&self, ray: &RayDiff) -> bool {
        self.shape.intersect_p(ray)
        
    }
    fn world_bound(&self) -> Bound<3> {
        self.shape.world_bound()
    }

    fn get_shape(&self)->Option<&dyn ShapeAble> {
       Some(self.shape.as_ref())
    }
}
impl Bounded for GeometricPrimitive {
    fn aabb(&self) -> ::bvh::aabb::AABB {
        let bound = self.shape.world_bound();
        bound.into()
    }
}
impl BHShape for GeometricPrimitive{
    fn set_bh_node_index(&mut self, i: usize) {
        self.node_index = i
    }
    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}