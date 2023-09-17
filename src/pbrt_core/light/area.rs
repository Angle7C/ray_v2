use std::f32::consts::PI;

use glam::Vec3;

use crate::pbrt_core::{primitive::{shape::Shape, Primitive}, tool::{SurfaceInteraction, InteractionCommon, Visibility, Bound}};

use super::LightAble;

pub trait AreaLight: LightAble + Primitive {
    fn l(&self, _surface: &InteractionCommon, _w: &Vec3) -> Vec3 {
        todo!()
    }
    fn get_shape(&self) -> &Shape;
}

#[derive(Debug)]
pub struct DiffuseAreaLight<'a> {
    lemit: Vec3,
    shape: &'a Shape<'a>,
    area: f32,
}

impl<'a> DiffuseAreaLight<'a> {
    pub fn new(lemit: Vec3, shape: &'a Shape<'a>) -> Self {
        Self {
            lemit,
            area: shape.agt_area(),
            shape,
        }
    }
}

impl<'a> AreaLight for DiffuseAreaLight<'a> {
    fn l(&self, surface: &InteractionCommon, w: &Vec3) -> Vec3 {
        if surface.normal.dot(*w) > 0.0 {
            self.lemit
        } else {
            Vec3::ZERO
        }
    }
    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}

impl<'a> LightAble for DiffuseAreaLight<'a> {
    fn power(&self) -> Vec3 {
        return self.area * PI * self.lemit;
    }
    fn sample_li(
        &self,
        surface: &SurfaceInteraction,
        //光源采样参数
        u: glam::Vec2,
        w_in: &mut Vec3,
        //光源pdf
        pdf: &mut f32,
        //可见性测试
        vis: &mut crate::pbrt_core::tool::Visibility,
    ) -> Vec3 {
        // 从shape采样到点
        let common = self.shape.sample(u);

        *w_in = (surface.common.p - common.p).normalize();

        *pdf = self.shape.pdf(&common, &w_in);
        *vis = Visibility { a: surface.common, b: common };
        self.l(&common, &-*w_in)
    }
    fn sample_le(&self) -> Vec3 {
        unimplemented!()
    }
    fn le(&self, wi: Vec3) -> Vec3 {
        let w = self.shape.get_mat().inverse().transform_vector3(wi);
        let cos = w.dot(Vec3::Z).clamp(0.0, 1.0);
        self.lemit * cos
    }
    fn pdf_li(&self, surface: &InteractionCommon, w_in: &Vec3) -> f32 {
        self.shape.pdf(surface, w_in)
    }
}

impl<'a> Primitive for DiffuseAreaLight<'a> {
    fn world_bound(&self) -> Bound<3> {
        self.shape.world_bound()
    }
    fn interacect(&self, ray: crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let mut inter = self.shape.interacect(ray);
        if let Some(ref mut suface) = inter {
            suface.light = self.get_light();
        }
        inter
    }
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: crate::pbrt_core::bxdf::TransportMode) {
        self.shape.compute_scattering(isct, mode)
    }
    fn get_light(&self) -> Option<&dyn LightAble> {
        Some(self)
    }
}