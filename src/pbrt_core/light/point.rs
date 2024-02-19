

use glam::{Vec2, Vec3};

use crate::pbrt_core::{
    primitive::{shape::ShapeAble},
    tool::{InteractionCommon, RayDiff, Visibility},
};
use crate::pbrt_core::light::LightType;
use crate::pbrt_core::tool::color::Color;

use super::LightAble;

#[derive(Debug, Clone, Copy)]
// #[derive(Debug,)]
pub struct Point {
    p: Vec3,
    lemit: Vec3,
    index:usize
}

impl Point {
    pub fn new(lemit: Vec3, p: Vec3,index:usize) -> Self {
        Self { p, lemit,index }
    }
}

impl LightAble for Point {
    fn get_samples(&self)->usize {
        1
    }

    #[inline]
    fn le(&self, _ray: &RayDiff,_shape:Option<&dyn ShapeAble>) -> Color {
        Color::ZERO
    }
    #[inline]
    fn pdf_li(&self, _surface: &InteractionCommon, _wi: &Vec3,_shape: Option<&dyn ShapeAble>,) -> f32 {
        0.0
    }
    #[inline]
    fn sample_li(&self,surface:&InteractionCommon,
            light_face:&mut InteractionCommon,
            _shape:Option<&dyn crate::pbrt_core::primitive::shape::ShapeAble>,
            _u:Vec2,
            wi:&mut Vec3,pdf:&mut f32,
            vis:&mut Visibility)->Color {
        *wi = (surface.p - self.p).normalize();
        *pdf = 1.0;
        light_face.p = self.p;
        light_face.time = surface.time;
        light_face.normal=-*wi;
        *vis = Visibility {
            a: *light_face,
            b: *surface,
        };
        (self.lemit/ self.p.distance_squared(surface.p)).into()
    }
    #[inline]
    fn get_type(&self) -> LightType {
        LightType::DeltaPosition
    }

    #[inline]
    fn li(&self, inter: &InteractionCommon, _w: &Vec3) -> Color {
        (self.lemit*(inter.p-self.p).length_recip()).into()
    }

    fn pdf_le(&self,_ray:&RayDiff,_normal:Vec3,_pdf_pos:&mut f32,_pdf_dir:&mut f32) {
        todo!()
    }
}
