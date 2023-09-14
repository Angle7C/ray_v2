use std::f32::consts::FRAC_1_PI;

use glam::Vec3;

use crate::pbrt_core::tool::color::Color;

use super::{
    func::{self, cos_theta},
    BxDFAble, BxDFType,
};
//慢反射
pub struct DisneyDiffuse {
    r: Color,
}
impl DisneyDiffuse {
    pub fn new(r: Color) -> Self {
        Self { r }
    }
}
impl BxDFAble for DisneyDiffuse {
    fn fi(&self, w_in: &Vec3, w_out: &Vec3) -> Vec3 {
        let fo = func::schlick_weight(cos_theta(w_out).abs());
        let fi = func::schlick_weight(cos_theta(w_in).abs());

        self.r * FRAC_1_PI * (1.0 - fo / 2.0) * (1.0 - fi / 2.0)
    }
    fn match_type(&self, flag: u32) -> bool {
        (BxDFType::Reflection | BxDFType::Diffuse) & flag > 0
    }
}
//电介质diff
pub struct DisneyRetro {
    r: Color,
    roughness: f32,
}
impl DisneyRetro {
    pub fn new(r: Color, roughness: f32) -> Self {
        Self { r, roughness }
    }
}

impl BxDFAble for DisneyRetro{
    fn fi(&self, w_in: &Vec3, w_out: &Vec3) -> Vec3 {
        let mut wh=*w_in+*w_out;
        if wh==Vec3::ZERO{
            return Vec3::ZERO;
        };
        wh=wh.normalize();
        let cos=w_in.dot(wh);
        let fo = func::schlick_weight(cos_theta(w_out).abs());
        let fi = func::schlick_weight(cos_theta(w_in).abs());
        let rr=2.0*self.roughness*cos*cos;

        self.r*FRAC_1_PI*rr*(fo+fi+fo*fi*(rr-1.0))
    }
    fn match_type(&self, flag: u32) -> bool {
        (BxDFType::Reflection | BxDFType::Diffuse) & flag > 0

    }
}
