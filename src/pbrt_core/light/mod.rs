use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr},
};

use glam::{Vec2, Vec3};

use self::area::AreaLight;

use super::{
    primitive::{shape:: Shape, Primitive},
    tool::{ InteractionCommon, SurfaceInteraction, Visibility},
};

pub mod area;
#[derive(Debug)]
pub enum Light {
    AreaLight(Box<dyn AreaLight>),
}
impl Primitive for Light{
    fn get_light(&self)->Option<&dyn LightAble>{
       match &self {
        Light::AreaLight(ref area) => area.get_light(),
    }
    }
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: super::bxdf::TransportMode) {
        match &self {
            Light::AreaLight(area) =>area.compute_scattering(isct, mode)
        }   
    }
    fn interacect(&self, ray: super::tool::RayDiff) -> Option<SurfaceInteraction> {
        match &self {
            Light::AreaLight(area) =>area.interacect(ray)
        }   
    }
    fn world_bound(&self) -> super::tool::Bound<3> {
        match &self {
            Light::AreaLight(area)=>area.world_bound()
        }
    }
}
impl Light {
    pub fn get_shape(&self) -> &Shape {
        match &self {
            Self::AreaLight(area) => area.get_shape(),
        }
    }
    pub fn pdf_li(&self, _inter: &InteractionCommon, _wi: &Vec3) -> f32 {
        unimplemented!()
    }
    pub fn le(&self, wi: &Vec3) -> Vec3 {
        match &self {
            Self::AreaLight(area) => area.le(*wi),
        }
    }
}
pub enum LightType {
    DeltaPosition = 1,
    DeltaDirection = 2,
    Area = 4,
    Infinite = 8,
}
impl LightType {
    fn _is_delta(flag: u32) -> bool {
        (flag & LightType::DeltaPosition > 0) || (flag & LightType::DeltaDirection > 0)
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
impl BitAnd<LightType> for LightType{
    type Output = u32;
    fn bitand(self, rhs: LightType) -> Self::Output {
        rhs as u32 &self as u32
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
impl BitOr<LightType> for LightType{
    type Output = u32;
    fn bitor(self, rhs: LightType) -> Self::Output {
        self as u32 | rhs as u32
    }
}

pub trait LightAble: Debug {
    ///对场景中得某一个点的 入射方向进行采样，会返回入射方向和光线pdf
    fn sample_li(
        &self,
        surface: &SurfaceInteraction,
        u: Vec2,
        w_in: &mut Vec3,
        pdf: &mut f32,
        vis: &mut Visibility,
    ) -> Vec3;
    /// 返回光源的光线，会返回入射方向和光线pdf
    fn sample_le(&self) -> Vec3;
    // 光源强度
    fn power(&self) -> Vec3;
    //pdf采样
    fn pdf_li(&self, surface: &InteractionCommon, w_in: &Vec3) -> f32;
    fn le(&self,wi:Vec3)->Vec3;
}
