use std::{fmt::Debug, ops::{BitAnd, BitOr}, todo};

use glam::{Vec2, Vec3};
use crate::pbrt_core::tool::color::Color;
use crate::pbrt_core::tool::{Ray, RayDiff};

use self::{area::AreaLight, infinite::InfiniteLight, point::Point};

use super::{
    primitive::{shape::Shape, Primitive},
    tool::{InteractionCommon, SurfaceInteraction, Visibility},
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
            Light::Infinite(ref inf) => inf.get_light(),
        }
    }
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: super::bxdf::TransportMode) {
        match &self {
            Light::AreaLight(area) => area.compute_scattering(isct, mode),
            Light::PointLight(point) => point.compute_scattering(isct, mode),
            Light::Infinite(ref infinite) => infinite.compute_scattering(isct, mode)
        }
    }
    fn interacect(&self, ray: super::tool::RayDiff) -> Option<SurfaceInteraction> {
        match &self {
            Light::AreaLight(area) => area.interacect(ray),
            Light::PointLight(point) => point.interacect(ray),
            Light::Infinite(ref infinite) => infinite.interacect(ray),
        }
    }
    fn world_bound(&self) -> super::tool::Bound<3> {
        match &self {
            Light::AreaLight(area) => area.world_bound(),
            Light::PointLight(point) => point.world_bound(),
            Light::Infinite(ref infinite) => infinite.world_bound(),
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
    pub fn pdf_li(&self, _inter: &InteractionCommon, _wi: &Vec3) -> f32 {
        unimplemented!()
    }
    pub fn le(&self, wi: &RayDiff) -> Vec3 {
        match &self {
            Self::AreaLight(area) => area.le(*wi),
            Self::Infinite(inf) => inf.le(*wi),
            _ => todo!(),
        }
    }
    pub fn get_type(&self) -> LightType {
        unimplemented!()
    }
}

impl LightAble for Light {
    fn sample_li(
        &self,
        surface_common: &InteractionCommon,
        light_common: &mut InteractionCommon,
        u: Vec2,
        wi: &mut Vec3,
        pdf: &mut f32,
        vis: &mut Visibility,
    ) -> Vec3 {
        todo!()
    }

    fn power(&self) -> Vec3 {
        todo!()
    }

    fn pdf_li(&self, surface: &SurfaceInteraction, wi: &Vec3) -> f32 {
        todo!()
    }

    fn le(&self, ray: RayDiff) -> Vec3 {
        todo!()
    }

    fn get_type(&self) -> LightType {
        todo!()
    }

    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color {
        todo!()
    }

    fn get_n_sample(&self) -> usize {
        todo!()
    }
}

pub trait LightAble: Debug + Primitive {
    ///对场景中得某一个点的 入射方向进行采样，会返回入射方向和光线pdf
    fn sample_li(
        &self,
        surface_common: &InteractionCommon,
        light_common: &mut InteractionCommon,
        u: Vec2,
        wi: &mut Vec3,
        pdf: &mut f32,
        vis: &mut Visibility,
    ) -> Vec3;
    /// 返回光源的光线，会返回入射方向和光线pdf
    // fn sample_le(&self,u1:Vec2,u2:Vec2,time:f32,ray:&mut Ray,light_n:&mut Vec3,pdf_pos:&mut f32,pdf_dir:&mut Vec3) -> Vec3;
    // fn pdf_le(&self,ray:&Ray,light_n:Vec3,pdf_pos:&mut f32,pdf_dir:&mut Vec3)->Vec3;
    // 光源强度
    fn power(&self) -> Vec3;
    //pdf采样
    fn pdf_li(&self, surface: &SurfaceInteraction, wi: &Vec3) -> f32;
    fn le(&self, ray: RayDiff) -> Vec3;
    fn get_type(&self) -> LightType;
    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color;

    fn get_n_sample(&self) -> usize;
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
            LightType::DeltaDirection => true,
            LightType::DeltaPosition => true,
            _ => false
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
