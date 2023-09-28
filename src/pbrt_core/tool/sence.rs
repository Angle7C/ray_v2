use std::sync::Arc;

use crate::pbrt_core::{
    camera::Camera,
    light::LightAble,
    material::MaterialAble,
    primitives::{bvh::{Accel, AccelAble}, shape::ShapeAble},
};

use super::{bound::Bound, interaction::SurfaceInteraction, ray::Ray};

pub struct Sence {
    pub light: Vec<Arc<dyn LightAble >>,
    pub accel: Option<Box<dyn AccelAble>>,
    pub camera: Camera,
    pub material: Vec<Arc<dyn MaterialAble>>,
    pub bound: Bound<3>,
    pub primitive: Vec<Arc<dyn ShapeAble>>,
}
impl Default for Sence {
    fn default() -> Self {
        Self {
            light: vec![],
            accel: None,
            camera: Camera::default(),
            material: vec![],
            bound: Default::default(),
            primitive: vec![],
        }
    }
}
unsafe impl Send for Sence {}
unsafe impl Sync for Sence {}
impl Sence {
    pub fn interacect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        if self.bound.intesect(ray) {
            if let Some(ref accel)=self.accel{
                accel.interacect(ray)
            }else{
                None
            }
        } else {
            None
        }
    }
}
