use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr},
};

use glam::{DVec2, DVec3};

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
    pub fn pdf_li(&self, _inter: &InteractionCommon, _wi: &DVec3) -> f64 {
        unimplemented!()
    }
    pub fn le(&self, wi: &DVec3) -> DVec3 {
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
        u: DVec2,
        w_in: &mut DVec3,
        pdf: &mut f64,
        vis: &mut Visibility,
    ) -> DVec3;
    /// 返回光源的光线，会返回入射方向和光线pdf
    fn sample_le(&self) -> DVec3;
    // 光源强度
    fn power(&self) -> DVec3;
    //pdf采样
    fn pdf_li(&self, surface: &InteractionCommon, w_in: &DVec3) -> f64;
    fn le(&self,wi:DVec3)->DVec3;
}
