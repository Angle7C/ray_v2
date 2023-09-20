use std::f32::consts::PI;

use glam::{Mat4, Vec3};

use crate::pbrt_core::{
    primitive::Primitive,
    tool::{Bound, InteractionCommon, Visibility},
};

use super::LightAble;

#[derive(Debug,Clone, Copy)]
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
    fn le(&self, wi: Vec3) -> Vec3 {
        self.lemit
    }
    fn pdf_li(&self, _surface: &crate::pbrt_core::tool::InteractionCommon, w_in: &Vec3) -> f32 {
        1.0
    }
    fn power(&self) -> Vec3 {
        self.lemit * 2.0 * PI
    }
    fn sample_le(&self,wi:Vec3,vis:&mut Visibility,surface: &InteractionCommon) -> Vec3 {
        unimplemented!()
    }
    fn sample_li(
        &self,
        surface: &crate::pbrt_core::tool::SurfaceInteraction,
        u: glam::Vec2,
        w_in: &mut Vec3,
        pdf: &mut f32,
        vis: &mut crate::pbrt_core::tool::Visibility,
    ) -> Vec3 {
        *w_in = (surface.common.p - self.p).normalize();

        *pdf = 1.0;
        *vis = Visibility {
            a: surface.common,
            b: InteractionCommon::new(-*w_in, self.p, *w_in, 0.0, Default::default()),
        };

        self.lemit
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
