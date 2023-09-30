

use glam::{Vec2, Vec3};

use crate::pbrt_core::{primitive::{shape::Shape, Primitive}, tool::{SurfaceInteraction, InteractionCommon, Visibility, Bound, RayDiff}};
use crate::pbrt_core::light::LightType;
use crate::pbrt_core::tool::color::Color;

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
    index:usize
}

impl<'a> DiffuseAreaLight<'a> {
    pub fn new(lemit: Vec3, shape: &'a Shape<'a>,index:usize) -> Self {
        Self {
            lemit,
            area: shape.agt_area(),
            shape,
            index
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
    fn get_n_sample(&self) -> usize {
        8
    }
    fn sample_li(&self, surface_common: &InteractionCommon, light_common: &mut InteractionCommon, u: Vec2, wi: &mut Vec3, pdf: &mut f32, vis: &mut Visibility) -> Vec3 {
        self.shape.sample(u,light_common,pdf);
        if pdf.abs()<f32::EPSILON || (light_common.p-surface_common.p).length_squared().abs()<f32::EPSILON{
            *pdf=0.0;
            Vec3::ZERO
        }else{
            *wi=(surface_common.p-light_common.p).normalize();
            *vis=Visibility{a:*light_common,b:*surface_common};
            self.l(light_common,&-*wi)
        }

    }
    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color {
        if inter.normal.dot(*w)>0.0 {
            self.lemit
        }else{
            Vec3::ZERO
        }
    }
    fn pdf_li(&self, surface: &SurfaceInteraction, wi: &Vec3) -> f32 {
        self.shape.pdf(&surface.common, wi)
    }
    fn get_type(&self) -> LightType {
        LightType::Area
    }
    fn get_index(&self)->usize {
        self.index   
    }
    fn le(&self,ray:RayDiff)->Color {
        if Vec3::Z.dot(ray.o.dir)>0.0{
            self.lemit
        }else{
            Color::ZERO
        }
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