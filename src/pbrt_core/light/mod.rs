use std::{
    fmt::Debug, ops::{BitAnd, BitOr}, todo
};

use crate::pbrt_core::tool::color::Color;
use crate::pbrt_core::tool::{RayDiff};
use bvh::ray::Ray;
use glam::{Vec2, Vec3};
use crate::pbrt_core::light::area::DiffuseAreaLight;

use self::{inf::InfiniteLight, point::Point};

use super::{
    primitive::shape::ShapeAble, tool::{InteractionCommon, Visibility}
};

pub mod area;
pub mod inf;
pub mod point;
pub mod spot;

pub trait LightAble {
    //光源采样
    fn sample_li(&self,surface:&InteractionCommon,
        light_face:&mut InteractionCommon,
        shape:Option<&dyn ShapeAble>,
        u:Vec2,
        wi:&mut Vec3,pdf:&mut f32,
        vis:&mut Visibility)->Color;
    //总能量
    fn power(&self)->Color{
        Color::ONE
    }
    //计算击中光源的radince 
    fn le(&self,ray:&RayDiff,shape:Option<&dyn ShapeAble>)->Color;
    //计算从face上完w_in方向进行采样的PDF
    fn pdf_li(&self,face:&InteractionCommon,w_in:&Vec3,shape: Option<&dyn ShapeAble>,)->f32;
    //进行radince采样
    fn sample_le(&self,_u1:Vec2,_u2:Vec2,_t:f32)->Option<Ray>{
        None
    }
    //获取光源类型
    fn get_type(&self)->LightType;
    //获取采样样本
    fn get_samples(&self)->usize{32}

    fn pdf_le(&self,ray:&RayDiff,normal:Vec3,pdf_pos:&mut f32,pdf_dir:&mut f32);

    fn li(&self,light_face:&InteractionCommon,wi:&Vec3)->Color;
}   

#[derive(Debug)]
pub enum Light {
    AreaLight(Box<DiffuseAreaLight>),
    PointLight(Box<Point>),
    Infinite(Box<InfiniteLight>),
}



impl LightAble for Light {
    fn sample_li(&self,surface:&InteractionCommon,
            light_face:&mut InteractionCommon,
            shape:Option<&dyn ShapeAble>,
            u:Vec2,
            wi:&mut Vec3,pdf:&mut f32,
            vis:&mut Visibility)->Color {
        match &self {
            Self::AreaLight(area) => area.sample_li(surface,light_face,shape,u,wi,pdf,vis),
            Self::Infinite(inf)=>inf.sample_li(surface,light_face,shape,u,wi,pdf,vis),
            Self::PointLight(point)=>point.sample_li(surface,light_face,shape,u,wi,pdf,vis),
        }
    }
    fn power(&self)->Color {
        match &self {
            Self::AreaLight(area) => area.power(),
            Self::Infinite(inf)=>inf.power(),
            Self::PointLight(point)=>point.power(),
        }
    }

    fn le(&self,ray:&RayDiff,shape:Option<&dyn ShapeAble>)->Color {
        match &self {
            Self::AreaLight(area) => area.le(ray,shape),
            Self::Infinite(inf)=>inf.le(ray,shape),
            Self::PointLight(point)=>point.le(ray,shape),
        }
    }

    fn pdf_li(&self,face:&InteractionCommon,w_in:&Vec3,shape: Option<&dyn ShapeAble>,)->f32 {
        match &self {
            Self::AreaLight(area) => area.pdf_li(face,w_in,shape),
            Self::Infinite(inf)=>inf.pdf_li(face,w_in,shape),
            Self::PointLight(point)=>point.pdf_li(face,w_in,shape),
        }
    }

    fn get_type(&self)->LightType {
        match &self {
            Self::AreaLight(area) => area.get_type(),
            Self::Infinite(inf)=>inf.get_type(),
            Self::PointLight(point)=>point.get_type(),
        }
    }

    fn get_samples(&self)->usize {
        match &self {
            Self::AreaLight(area) => area.get_samples(),
            Self::Infinite(inf)=>inf.get_samples(),
            Self::PointLight(point)=>point.get_samples(),
        }
    }

    fn pdf_le(&self,_ray:&RayDiff,_normal:Vec3,_pdf_pos:&mut f32,_pdf_dir:&mut f32) {
        todo!()
    }

    fn li(&self,_light_face:&InteractionCommon,_wi:&Vec3)->Color {
        todo!()
    }
}

pub enum LightType {
    DeltaPosition = 1,
    DeltaDirection = 2,
    Area = 4,
    Infinite = 8,
}

impl LightType {
    pub fn is_delta(flag: LightType) -> bool {
    
        matches!(flag,LightType::DeltaDirection|LightType::DeltaPosition)
    }
    pub fn is_inf(flag: LightType)->bool{
       matches!(flag,LightType::Infinite)
    }
}

impl BitAnd<u32> for LightType {
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        rhs & self as u32
    }
}

impl BitOr<u32> for LightType {
    type Output = u32;
    fn bitor(self, rhs: u32) -> Self::Output {
        rhs | self as u32
    }
}

impl BitAnd<LightType> for LightType {
    type Output = u32;
    fn bitand(self, rhs: LightType) -> Self::Output {
        rhs as u32 & self as u32
    }
}

impl BitAnd<LightType> for u32 {
    type Output = u32;
    fn bitand(self, rhs: LightType) -> Self::Output {
        rhs & self
    }
}

impl BitOr<LightType> for u32 {
    type Output = u32;
    fn bitor(self, rhs: LightType) -> Self::Output {
        rhs | self
    }
}

impl BitOr<LightType> for LightType {
    type Output = u32;
    fn bitor(self, rhs: LightType) -> Self::Output {
        self as u32 | rhs as u32
    }
}
