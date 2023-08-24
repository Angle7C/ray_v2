use std::f64::consts::PI;

use glam::DVec3;

use crate::pbrt_core::{tool::SurfaceInteraction, primitive::shape::Shape};

use super::LightAble;

pub trait AreaLight:LightAble{
    fn l(&self,surface:&SurfaceInteraction,w:&DVec3)->DVec3{
        todo!()
    }
    fn le(&self,w:DVec3)->DVec3{
        todo!()
    }
}

pub struct DiffuseAreaLight{
    lemit:DVec3,
    shape:Shape,
    area:f64
}
impl AreaLight for DiffuseAreaLight{
    fn l(&self,surface:&SurfaceInteraction,w:&DVec3)->DVec3 {
        if surface.common.normal.dot(*w)>0.0{
            self.lemit
        }else{
            DVec3::ZERO
        }
    }
    fn le(&self,w:DVec3)->DVec3 {
        w
    }
}
impl LightAble for DiffuseAreaLight{
    fn power(&self)->DVec3 {
        return self.area*PI*self.lemit;
    }
    fn sample_f(&self,surface:&SurfaceInteraction,u:glam::DVec2,w_in:&mut DVec3,pdf:& mut f64,vis:&mut crate::pbrt_core::tool::Visibility)->DVec3 {
        todo!()
    }
}