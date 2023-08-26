use glam::f64::DVec3;

use super::{
    frensnel::Fresnel,
    func::{face_forward, refract},
    BxDFAble, BxDFType, TransportMode,
};
//镜面反射
pub struct SpecularReflection {
    //光谱颜色
    r: DVec3,
    frensnel: Fresnel,
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
    fn sample_fi(
        &self,
        w_in: &mut DVec3,
        _w_out: glam::DVec3,
        _sample_point: glam::DVec2,
    ) -> glam::DVec3 {
        let cos_theta_i = w_in.z;
        self.frensnel.evaluate(cos_theta_i) / w_in.z.abs()
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
    fn sample_fi(&self, w_in: &mut DVec3, w_out: DVec3, _sample_point: glam::DVec2) -> DVec3 {
        //入射
        let (eta_i, eta_t) = if w_out.z > 0.0 {
            (self.eta_a, self.eta_b)
        //出射
        } else {
            (self.eta_b, self.eta_a)
        };
        if !refract(&w_out, &face_forward(DVec3::Z, w_out), eta_i / eta_t, w_in) {
            DVec3::ZERO
        } else {
            self.t
                * (DVec3::ONE - self.fresnel.evaluate(w_in.z))
                * if self.mode == TransportMode::Radiance {
                    (eta_i * eta_i) / (eta_t * eta_t)
                } else {
                    1.0
                }
                / w_in.z.abs()
        }
    }
}
