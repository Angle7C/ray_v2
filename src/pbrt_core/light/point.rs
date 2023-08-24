use std::f64::consts::PI;

use glam::{DVec3, DMat4};

use crate::pbrt_core::tool::Visibility;

use super::LightAble;

pub struct PointLight{
    i:DVec3,
    p:DVec3,
    object_to_world:DMat4
}
impl PointLight{
    pub fn new(i:DVec3,p:DVec3,object_to_world:DMat4)->Self{
        Self { i, p, object_to_world }
    }
}
impl LightAble for PointLight{
    fn power(&self)->DVec3 {
        4.0*PI*self.i
    }
    fn sample_f(&self,surface:&crate::pbrt_core::tool::SurfaceInteraction,u:glam::DVec2,w_in:&mut DVec3,pdf:& mut f64,vis:&mut Visibility)->DVec3 {
        *w_in=(self.p-surface.common.p).normalize();
        *pdf=1.0;
        *vis=Visibility{p1:self.p,p2:surface.common.p};
        self.i/(self.p-surface.common.p).length_squared()
    }
}
pub struct SpotLight{
    p:DVec3,
    i:DVec3,
    cos_width:f64,
    cos_start:f64
}
impl LightAble for SpotLight{
    fn power(&self)->DVec3 {
        self.i*(2.0*PI*(1.0-0.5*(self.cos_start+self.cos_width)))
    }

    fn sample_f(&self,surface:&crate::pbrt_core::tool::SurfaceInteraction,u:glam::DVec2,w_in:&mut DVec3,pdf:& mut f64,vis:&mut Visibility)->DVec3 {
        *w_in=(self.p-surface.common.p).normalize();
        *pdf=1.0;
        *vis=Visibility{p1:self.p,p2:surface.common.p};
        self.i*self.fall_off(&w_in)/(self.p-surface.common.p).length_squared()
    }
}
impl SpotLight {
    pub fn fall_off(&self,wi:&DVec3)->f64{
        let cos_theta=wi.z;
        if cos_theta<self.cos_width{
            return 0.0;
        }
        if cos_theta>self.cos_start{
            return 1.0;
        }
        let delta=(cos_theta-self.cos_width)/(self.cos_start-self.cos_width);
        return (delta*delta)*(delta*delta);
    }
}