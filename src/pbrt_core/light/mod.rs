use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr},
};

use glam::{Vec2, Vec3};
use crate::pbrt_core::tool::color::Color;
use crate::pbrt_core::tool::Ray;

use self::{area::AreaLight, infinite::InfiniteLight, point::Point};

use super::{
    primitive::{shape::Shape, Primitive},
    tool::{InteractionCommon, SurfaceInteraction, Visibility, RayDiff},
};

pub mod area;
pub mod infinite;
pub mod point;
pub mod spot;
#[derive(Debug)]
pub enum Light {
    AreaLight(Box<dyn AreaLight>),
    PointLight(Box<Point>),
    Infinite(Box<InfiniteLight>),
}
impl Primitive for Light {
    fn get_light(&self) -> Option<&dyn LightAble> {
        match &self {
            Light::AreaLight(ref area) => area.get_light(),
            Light::PointLight(ref point) => point.get_light(),
            Light::Infinite(ref inf)=>inf.get_light(),
        }
    }
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: super::bxdf::TransportMode) {
        match &self {
            Light::AreaLight(area) => area.compute_scattering(isct, mode),
            Light::PointLight(point) => point.compute_scattering(isct, mode),
            Light::Infinite(ref infinite)=>infinite.compute_scattering(isct,mode)
        }
    }
    fn interacect(&self, ray: super::tool::RayDiff) -> Option<SurfaceInteraction> {
        match &self {
            Light::AreaLight(area) => area.interacect(ray),
            Light::PointLight(point) => point.interacect(ray),
            Light::Infinite(ref infinite)=>infinite.interacect(ray),

        }
    }
    fn world_bound(&self) -> super::tool::Bound<3> {
        match &self {
            Light::AreaLight(area) => area.world_bound(),
            Light::PointLight(point) => point.world_bound(),
            Light::Infinite(ref infinite)=>infinite.world_bound(),

        }
    }
    
}
impl Light {
    pub fn get_shape(&self) -> &Shape {
        match &self {
            Self::AreaLight(area) => area.get_shape(),
            _ => todo!(),
        }
    }
}
impl LightAble for Light {
    fn sample_li(
        &self,
        surface_common: &InteractionCommon,
        light_common:&mut InteractionCommon,
        u: Vec2,
        wi: &mut Vec3,
        pdf: &mut f32,
        vis: &mut Visibility,
    ) -> Vec3 {
        match self {
            Light::AreaLight(area) => area.sample_li(surface_common, light_common, u, wi, pdf, vis),
            Light::PointLight(p) => p.sample_li(surface_common, light_common, u, wi, pdf, vis),
            Light::Infinite(inf) => inf.sample_li(surface_common, light_common, u, wi, pdf, vis)
        }
    }

    fn power(&self) -> Vec3 {
        match self {
            Light::AreaLight(area) => area.power(),
            Light::PointLight(p) => p.power(),
            Light::Infinite(inf) => inf.power(),
        }
    }

    fn pdf_li(&self, surface: &SurfaceInteraction, wi: &Vec3) -> f32 {
        match self {
            Light::AreaLight(area) => area.pdf_li(surface, wi),
            Light::PointLight(p) => p.pdf_li(surface, wi),
            Light::Infinite(inf) => inf.pdf_li(surface, wi),
        }
    }

    fn le(&self, ray:RayDiff) -> Vec3 {
        match self {
            Light::AreaLight(area) => area.le(ray),
            Light::PointLight(p) => p.le(ray),
            Light::Infinite(inf) => inf.le(ray),
        }
    }

    fn get_type(&self)->LightType {
        match self {
            Light::AreaLight(area) => area.get_type(),
            Light::PointLight(p) => p.get_type(),
            Light::Infinite(inf) => inf.get_type(),
        }
    }

    fn li(&self,inter:&InteractionCommon,w:&Vec3)->Color {
        todo!()
    }

    fn get_n_sample(&self)->usize {
        todo!()
    }
}

pub trait LightAble: Debug + Primitive {
    ///对场景中得某一个点的 入射方向进行采样，会返回入射方向和光线pdf
    fn sample_li(
        &self,
        surface_common: &InteractionCommon,
        light_common:&mut InteractionCommon,
        u: Vec2,
        wi: &mut Vec3,
        pdf: &mut f32,
        vis: &mut Visibility,
    ) -> Vec3;
    // 光源强度
    fn power(&self) -> Vec3;
    //pdf采样
    fn pdf_li(&self, surface: &SurfaceInteraction, wi: &Vec3) -> f32;
    fn le(&self, ray:RayDiff) -> Vec3;
    fn get_type(&self)->LightType;
    fn li(&self,inter:&InteractionCommon,w:&Vec3)->Color;

    fn get_n_sample(&self)->usize;
}
pub enum LightType {
    DeltaPosition = 1,
    DeltaDirection = 2,
    Area = 4,
    Infinite = 8,
}
impl LightType {
    pub fn is_delta(flag: LightType) -> bool {
        match flag {
            LightType::DeltaDirection=>true,
            LightType::DeltaPosition=>true,
            _=>false
        }
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
