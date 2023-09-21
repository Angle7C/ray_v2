use std::f32::consts::PI;

use glam::{Vec2, Vec3};

use crate::pbrt_core::{primitive::{shape::Shape, Primitive}, tool::{SurfaceInteraction, InteractionCommon, Visibility, Bound}};
use crate::pbrt_core::light::LightType;
use crate::pbrt_core::tool::color::Color;
use crate::pbrt_core::tool::{Ray, RayDiff};

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
    fn get_n_sample(&self) -> usize {
        8
    }
    fn sample_li(&self, surface_common: &InteractionCommon, light_common: &mut InteractionCommon, u: Vec2, wi: &mut Vec3, pdf: &mut f32, vis: &mut Visibility) -> Vec3 {
        *light_common = self.shape.sample(u,surface_common,pdf);
        if pdf.abs()==f32::EPSILON || (light_common.p-surface_common.p).length_squared().abs()<f32::EPSILON{
            *pdf=0.0;
            Vec3::ZERO
        }else{
            *wi=(light_common.p-surface_common.p).normalize();
            *vis=Visibility{a:*surface_common,b:*light_common};
            self.l(light_common,&-*wi)
        }

    }
    fn le(&self, ray: RayDiff) -> Vec3 {
        todo!()
    }
    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color {
        if inter.normal.dot(w)>0.0 {
            self.lemit
        }else{
            Vec3::ZERO
        }
    }
    fn pdf_li(&self, surface: &InteractionCommon, w_in: &Vec3) -> f32 {
        self.shape.pdf(surface, w_in)
    }
    fn get_type(&self) -> LightType {
        LightType::Infinite
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