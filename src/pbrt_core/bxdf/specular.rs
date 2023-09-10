use std::ops::Mul;

use glam::f64::DVec3;

use super::{frensnel::Fresnel, func::cos_theta, BxDFAble, BxDFType, TransportMode};
//镜面反射
pub struct SpecularReflection {
    //光谱颜色
    r: DVec3,
    frensnel: Fresnel,
}
impl SpecularReflection {
    pub fn new(r: DVec3, frensnel: Fresnel) -> Self {
        Self { r, frensnel }
    }
}
impl BxDFAble for SpecularReflection {
    //对于任意一对（w_in,w_out）都是0
    fn fi(&self, _w_in: &glam::DVec3, _w_out: &glam::DVec3) -> glam::DVec3 {
        DVec3::ZERO
    }
    //反射与高光
    fn match_type(&self, flag: u32) -> bool {
        ((BxDFType::Specular | BxDFType::Reflection) & flag as u32) != 0
    }
    //
    fn rho(
        &self,
        _w_in: glam::DVec3,
        _w_out: glam::DVec3,
        _sample_point: glam::DVec2,
    ) -> glam::DVec3 {
        DVec3::ZERO
    }
    fn sample_f(
        &self,
        w_in: &mut DVec3,
        w_out: &DVec3,
        sample_point: glam::DVec2,
        pdf: &mut f64,
    ) -> DVec3 {
        *w_in = w_out.mul(DVec3::new(-1.0, -1.0, 1.0));
        *pdf = 1.0;
        let cos_i = cos_theta(&w_in);
        self.frensnel.evaluate(cos_i) * self.r / cos_theta(&w_in).abs()
    }
}
//镜面透射
pub struct SpecularTransmission {
    //上方折射率
    eta_a: f64,
    //下方折射率
    eta_b: f64,
    fresnel: Fresnel,
    mode: TransportMode,
    t: DVec3,
}
impl BxDFAble for SpecularTransmission {
    fn fi(&self, _w_in: &DVec3, _w_out: &DVec3) -> DVec3 {
        DVec3::ZERO
    }
    fn match_type(&self, flag: u32) -> bool {
        ((BxDFType::Specular | BxDFType::Transmission) & flag as u32) != 0
    }
    fn rho(&self, _w_in: DVec3, _w_out: DVec3, _sample_point: glam::DVec2) -> DVec3 {
        DVec3::ZERO
    }
}
