use std::f32::consts::PI;

use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    primitive::Primitive,
    tool::{Bound, InteractionCommon, Visibility, RayDiff},
};
use crate::pbrt_core::light::LightType;
use crate::pbrt_core::tool::color::Color;

use super::LightAble;

#[derive(Debug, Clone, Copy)]
// #[derive(Debug,)]
pub struct Point {
    p: Vec3,
    lemit: Vec3,
    // object_to_wworld:Mat4
}

impl Point {
    pub fn new(lemit: Vec3, p: Vec3, _object_to_wworld: Mat4) -> Self {
        Self { p, lemit }
    }
}

impl LightAble for Point {
    #[inline]
    fn le(&self, ray: RayDiff) -> Vec3 {
        Vec3::ZERO
    }
    #[inline]
    fn pdf_li(&self, surface: &crate::pbrt_core::tool::SurfaceInteraction, wi: &Vec3) -> f32 {
        0.0
    }
    #[inline]
    fn power(&self) -> Vec3 {
        self.lemit * 4.0 * PI
    }
    #[inline]
    fn sample_li(&self, surface_common: &InteractionCommon, light_common: &mut InteractionCommon, u: Vec2, wi: &mut Vec3, pdf: &mut f32, vis: &mut Visibility) -> Vec3 {
        *wi = (surface_common.p - self.p).normalize();
        *pdf = 1.0;
        light_common.p = self.p;
        light_common.time = surface_common.time;
        *vis = Visibility { a: *surface_common, b: *light_common };
        self.lemit/ self.p.distance_squared(surface_common.p)
    }
    #[inline]
    fn get_type(&self) -> LightType {
        LightType::DeltaPosition
    }
    #[inline]
    fn get_n_sample(&self) -> usize {
        1
    }
    #[inline]
    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color {
        self.lemit*(inter.p-self.p).length_recip()
    }
}

impl Primitive for Point {
    fn get_area(&self) -> f32 {
        1.0
    }
    fn get_light(&self) -> Option<&dyn super::LightAble> {
        Some(self)
    }
    fn interacect(
        &self,
        _ray: crate::pbrt_core::tool::RayDiff,
    ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
        None
    }
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let bound = Bound::<3>::new(Vec3::splat(-0.0003) + self.p, Vec3::splat(0.0003) + self.p);
        bound
    }
}
