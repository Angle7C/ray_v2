use std::ops::Mul;

use glam::f32::Vec3;

use crate::pbrt_core::tool::color::Color;

use super::{frensnel::Fresnel, func::cos_theta, BxDFAble, BxDFType, TransportMode};
//镜面反射
pub struct SpecularReflection {
    //光谱颜色
    r: Color,
    //菲涅尔项
    frensnel: Fresnel,
}
impl SpecularReflection {
    pub fn new(r: Color, frensnel: Fresnel) -> Self {
        Self { r, frensnel }
    }
}
impl BxDFAble for SpecularReflection {
    //对于任意一对（w_in,w_out）都是0
    fn f(&self, _w_in: &glam::Vec3, _w_out: &glam::Vec3) -> Color {
        Color::ZERO
    }
    //反射与高光
    fn match_type(&self, flag: u32) -> bool {
        ((BxDFType::Specular | BxDFType::Reflection) & flag) != 0
    }
    fn sample_f(
        &self,
        w_in: &mut Vec3,
        w_out: &Vec3,
        _sample_point: glam::Vec2,
        pdf: &mut f32,
    ) -> Color {
        *w_in = w_out.mul(Vec3::new(-1.0, -1.0, 1.0));
        *pdf = 1.0;
        let cos_i = cos_theta(w_in);
        (self.r / cos_theta(w_in).abs()) * self.frensnel.evaluate(cos_i) 
    }
    fn get_type(&self)->u32 {
        BxDFType::Specular | BxDFType::Reflection
    }
}
//镜面透射
pub struct SpecularTransmission {
    //上方折射率
    _eta_a: f32,
    //下方折射率
    _eta_b: f32,
    _fresnel: Fresnel,
    _mode: TransportMode,
    _t: Vec3,
}
impl BxDFAble for SpecularTransmission {
    fn f(&self, _w_in: &Vec3, _w_out: &Vec3) -> Color {
        Color::ZERO
    }
    fn match_type(&self, flag: u32) -> bool {
        ((BxDFType::Specular | BxDFType::Transmission) & flag) != 0
    }
    fn get_type(&self)->u32 {
        BxDFType::Specular | BxDFType::Transmission
    }
}
