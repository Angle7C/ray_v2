use std::f64::consts::PI;

use glam::{DVec3, DVec2, Vec3};

use crate::pbrt_core::{primitive::shape::Shape, tool::{SurfaceInteraction, InteractionCommon, func::vec3_coordinate_system}, sampler::cosine_sample_hemisphere};

use super::LightAble;

pub trait AreaLight: LightAble {
    fn l(&self, surface: &SurfaceInteraction, w: &DVec3) -> DVec3 {
        todo!()
    }
    fn le(&self, w: DVec3) -> DVec3 {
        todo!()
    }
    fn get_shape(&self)->&Shape;
}
#[derive(Debug)]
pub struct DiffuseAreaLight {
    lemit: DVec3,
    shape: Shape,
    area: f64,
}
impl DiffuseAreaLight {
    pub fn new(lemit: DVec3, shape: Shape) -> Self {
        Self {
            lemit,
            area: shape.age_area(),
            shape,
        }
    }
}
impl AreaLight for DiffuseAreaLight {
    fn l(&self, surface: &SurfaceInteraction, w: &DVec3) -> DVec3 {
        if surface.common.normal.dot(*w) > 0.0 {
            self.lemit
        } else {
            DVec3::ZERO
        }
    }
    fn le(&self, w: DVec3) -> DVec3 {
        w
    }
    fn get_shape(&self)->&Shape {
        &self.shape
    }
}
impl LightAble for DiffuseAreaLight {
    fn power(&self) -> DVec3 {
        return self.area * PI * self.lemit;
    }
    fn sample_f(
        &self,
        surface: &SurfaceInteraction,
        u: glam::DVec2,
        w_in: &mut DVec3,
        pdf: &mut f64,
        vis: &mut crate::pbrt_core::tool::Visibility,
    ) -> DVec3 {
        // self.
        let common=self.shape.sample(u);
        let light_n=common.normal;
        let mut w=cosine_sample_hemisphere(u);
        *pdf=w.z;
        let mut s = SurfaceInteraction::default();
       
        let mut v1=Default::default();
        let mut v2=Default::default();
        vec3_coordinate_system(common.normal, &mut v1,&mut v2);
        w=v1*w.x+v2*w.y+common.normal*w.z;
        s.common=common;
        self.l(&s, &w)
    }
   
}
