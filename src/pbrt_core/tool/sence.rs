
use crate::pbrt_core::{
    camera::Camera, light::LightAble, material::MaterialAble, primitives::bvh::Accel,
};

use super::{bound::Bound, interaction::SurfaceInteraction, ray::Ray};

pub struct Sence<'a> {
    pub light: Vec<Box<dyn LightAble+'a>>,
    pub accel: Accel<'a>,
    pub camera: Camera,
    pub material: Vec<Box<dyn MaterialAble+'a>>,
    pub bound: Bound<3>,
}
unsafe impl<'a> Send for Sence<'a>{}
unsafe impl<'a> Sync for Sence<'a>{}
impl<'a> Sence<'a> {
    pub fn interacect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        if self.bound.intesect(ray) {
            self.accel.interacect(ray)
        }else{
            None
        }
    }
}
