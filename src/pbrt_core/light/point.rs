use std::f32::consts::PI;

use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    primitive::Primitive,
    tool::{Bound, InteractionCommon, Visibility},
};
use crate::pbrt_core::light::LightType;
use crate::pbrt_core::tool::color::Color;
use crate::pbrt_core::tool::{Ray, RayDiff};

use super::LightAble;

#[derive(Debug, Clone, Copy)]
// #[derive(Debug,)]
pub struct Point {
    p: Vec3,
    lemit: Vec3,
    // object_to_wworld:Mat4
}

impl Point {
    pub fn new(lemit: Vec3, p: Vec3, object_to_wworld: Mat4) -> Self {
        Self { p, lemit }
    }
}

impl LightAble for Point {
    fn pdf_li(&self, surface: &crate::pbrt_core::tool::InteractionCommon, w_in: &Vec3) -> f32 {
        0.0
    }
    fn le(&self, ray: RayDiff) -> Vec3 {
        Vec3::ZERO
    }
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
    fn get_type(&self) -> LightType {
        LightType::DeltaPosition
    }
    fn get_n_sample(&self) -> usize {
        1
    }
    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color {
        unimplemented!()
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
        //     let min=self.object_to_wworld.transform_point3(bound.min);
        //     let max=self.object_to_wworld.transform_point3(bound.max);
        //     Bound { min: min, max: max }
        bound
        // }
    }
}
