use std::{ops::{BitAnd, BitOrAssign, BitOr}, fmt::Debug};

use glam::{DVec2, DVec3};
use gltf::json::extras::Void;

use self::{point::PointLight, area::AreaLight};

use super::{tool::{SurfaceInteraction, Visibility}, primitive::shape::{self, Shape}};

pub mod point;
pub mod area;
#[derive(Debug)]
pub enum Light{
    PointLight(Box<PointLight>),
    AreaLight(Box<dyn AreaLight>)
}
impl Light{
    pub fn get_shape(&self)->&Shape{
        match &self {
            Self::PointLight(point)=>unimplemented!(),
            Self::AreaLight(area)=>area.get_shape()
        }
    }
}
pub enum LightType{
    DeltaPosition = 1,
    DeltaDirection = 2,
    Area=4,
    Infinite=8
}
impl LightType{
    fn is_delta(flag:u32)->bool{
        (flag & LightType::DeltaPosition>0) || (flag& LightType::DeltaDirection>0) 
    }
}
impl BitAnd<u32> for LightType{
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        rhs & self as u32
    }
}
impl BitOr<u32> for LightType{
    type Output=u32;
    fn bitor(self, rhs: u32) -> Self::Output {
        rhs | self as u32
    }
}
impl BitAnd<LightType> for u32{
    type Output = u32;
    fn bitand(self, rhs: LightType) -> Self::Output {
        rhs&self
    }
}
impl BitOr<LightType> for u32{
    type Output=u32;
    fn bitor(self, rhs: LightType) -> Self::Output {
        rhs|self
    }
}
pub trait LightAble:Debug {
    fn sample_f(&self,surface:&SurfaceInteraction,u:DVec2,w_in:&mut DVec3,pdf:& mut f64,vis:&mut Visibility)->DVec3;
    fn power(&self)->DVec3;
}