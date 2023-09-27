use glam::Vec3A;

use crate::pbrt_core::{
    primitives::shape::{Shape, ShapeAble},
    tool::{color::Color, interaction::InteractionCommon},
};

use super::LightAble;

pub struct DiffuseAreaLight<'a> {
    lemit: Color,
    shape: &'a dyn ShapeAble,
    area: f32,
}
impl<'a> DiffuseAreaLight<'a> {
    pub fn new(lemit: Vec3A, shape: &'a dyn ShapeAble) -> Self {
        Self {
            lemit:lemit.into(),
            shape,
            area: shape.area(),
        }
    }
}
impl<'a> DiffuseAreaLight<'a> {
    fn l(&self, inter: &InteractionCommon, w: &Vec3A) -> Color {
        if inter.n.dot(*w) > 0.0 {
            self.lemit
        } else {
            Color::ZERO
        }
    }
}
impl<'a> LightAble for DiffuseAreaLight<'a> {
    fn get_n_samples(&self) -> usize {
        8
    }
    fn get_type(&self) -> u32 {
        super::LightType::Area as u32
    }
    fn pdf_li(
        &self,
        // vis: &mut crate::pbrt_core::tool::vistest::VisibilityTester,
        inter: &crate::pbrt_core::tool::interaction::SurfaceInteraction,
        wi: Vec3A,
    ) -> f32 {
        1.0 / self.area
    }
    fn sample_li(
        &self,
        common: &crate::pbrt_core::tool::interaction::InteractionCommon,
        light_common: &mut crate::pbrt_core::tool::interaction::InteractionCommon,
        u: &glam::Vec2,
        wi: &mut Vec3A,
        pdf: &mut f32,
        vis: &mut crate::pbrt_core::tool::vistest::VisibilityTester,
    ) -> crate::pbrt_core::tool::color::Color {
        unimplemented!()
    }
}
