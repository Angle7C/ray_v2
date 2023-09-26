
use crate::pbrt_core::{
    camera::Camera, light::LightAble, material::MaterialAble, primitives::bvh::Accel,
};

use super::{bound::Bound, interaction::SurfaceInteraction, ray::Ray};

pub struct Sence<'a> {
    pub light: Vec<Box<dyn LightAble>>,
    pub accel: Accel<'a>,
    pub camera: Camera,
    pub material: Vec<Box<dyn MaterialAble>>,
    pub bound: Bound<3>,
}
impl<'a> Sence<'a> {
    pub fn interacect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        if self.bound.intesect(ray) {
            self.accel.interacect(ray)
        }else{
            None
        }
    }
}
