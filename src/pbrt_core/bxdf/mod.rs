use std::f32::consts::FRAC_1_PI;

use glam::{Vec2, Vec3A};

use self::func::{same_hemisphere, cos_theta};

use super::{sampler::cosine_sample_hemisphere, tool::color::Color};

pub mod diffuse;

pub mod specular;

pub mod fresnel;

pub trait BxDFAble {
    fn match_type(&self, rhs: u32) -> bool {
        self.get_type() & rhs > 0
    }
    ///给定两个方向， 返回BSDF值，
    fn f(&self, wo: &Vec3A, wi: &Vec3A) -> Color;
    fn sample_f(
        &self,
        wo: &Vec3A,
        wi: &mut Vec3A,
        u_point: &Vec2,
        pdf: &mut f32,
        sampled_type: Option<&mut u32>,
    ) -> Color {
        *wi = cosine_sample_hemisphere(*u_point).into();
        if wo.z < 0.0 {
            wi.z = -wi.z
        }
        *pdf = self.pdf(wo, wi);
        self.f(wo, wi)
    }
    fn get_type(&self) -> u32;
    fn pdf(&self, wo: &Vec3A, wi: &Vec3A) ->f32{
        if same_hemisphere(wo, wi){
            cos_theta(wi).abs()*FRAC_1_PI
        }else{
            0.0
        }
    }
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum BxDFType {
    Reflection = 1,
    Transmission = 2,
    Diffuse = 4,
    Glossy = 8,
    Specular = 16,
    All = 31,
}
impl From<BxDFType> for u32 {
    fn from(value: BxDFType) -> Self {
        match value {
            BxDFType::Reflection => 1,
            BxDFType::Transmission => 2,
            BxDFType::Diffuse => 4,
            BxDFType::Glossy => 8,
            BxDFType::Specular => 16,
            BxDFType::All => 31,
        }
    }
}
#[allow(unused)]
mod func {
    use glam::Vec3A;

    use crate::pbrt_core::tool::color::Color;

    #[inline]
    pub fn _fr_dielectric(cos_theta_i: f32, eta_i: f32, eta_t: f32) -> f32 {
        unimplemented!()
    }
    #[inline]
    pub fn _fr_conductor(cos_theta_i: f32, eta_i: &Vec3A, eta_t: &Vec3A, k: &Vec3A) -> Color {
        unimplemented!()
    }
    #[inline]
    pub fn cos_theta(w: &Vec3A) -> f32 {
        debug_assert!(w.is_normalized(), "it's not unit Vec3");
        w.z
    }
    #[inline]
    pub fn cos_2_theta(w: &Vec3A) -> f32 {
        debug_assert!(w.is_normalized(), "it's not unit Vec3");
        w.z * w.z
    }
    #[inline]
    pub fn cos_theta_abs(w: &Vec3A) -> f32 {
        w.z.abs()
    }
    #[inline]
    pub fn sin_2_theta(w: &Vec3A) -> f32 {
        (1.0 - cos_2_theta(w)).max(0.0)
    }
    #[inline]
    pub fn sin_theta(w: &Vec3A) -> f32 {
        sin_2_theta(w).sqrt()
    }
    #[inline]
    pub fn tan_theta(w: &Vec3A) -> f32 {
        sin_theta(w) / cos_2_theta(w)
    }
    #[inline]
    pub fn tan_2_theta(w: &Vec3A) -> f32 {
        sin_2_theta(w) / cos_2_theta(w)
    }
    #[inline]
    pub fn cos_phi(w: &Vec3A) -> f32 {
        let sin_theta = sin_theta(w);
        if sin_theta < f32::EPSILON {
            1.0
        } else {
            (w.x / sin_theta).clamp(-1.0, 1.0)
        }
    }
    #[inline]
    pub fn sin_phi(w: &Vec3A) -> f32 {
        let sin_theta = sin_theta(w);
        if sin_theta < f32::EPSILON {
            1.0
        } else {
            (w.y / sin_theta).clamp(-1.0, 1.0)
        }
    }
    #[inline]
    pub fn cos_2_phi(w: &Vec3A) -> f32 {
        cos_phi(w) * cos_phi(w)
    }
    #[inline]
    pub fn sin_2_phi(w: &Vec3A) -> f32 {
        sin_phi(w) * sin_phi(w)
    }
    #[inline]
    pub fn cos_d_phi(wa: &Vec3A, wb: &Vec3A) -> f32 {
        let waxy = wa.x * wa.x + wb.y * wb.y;
        let wbxy = wb.x * wb.x + wb.y * wb.y;
        if waxy.abs() < f32::EPSILON || wbxy.abs() < f32::EPSILON {
            1.0
        } else {
            ((wa.x * wb.x + wa.y * wb.y) / (waxy * wbxy).sqrt()).clamp(-1.0, 1.0)
        }
    }
    #[inline]
    pub fn reflect(wo: &Vec3A, n: Vec3A) -> Vec3A {
        -*wo + 2.0 * wo.dot(n) * n
    }
    #[inline]
    pub fn same_hemisphere(w: &Vec3A, wp: &Vec3A) -> bool {
        w.z * wp.z > 0.0
    }
    #[inline]
    pub fn refract(wi: &Vec3A, n: &Vec3A, eta: f32) -> Option<Vec3A> {
        let cos_theta_i = n.dot(*wi);
        let sin_2_theta_i = (1.0 - cos_theta_i * cos_theta_i).clamp(-1.0, 1.0);
        let sin_2_theta_t = eta * eta * sin_2_theta_i;
        if sin_2_theta_t > 1.0 {
            None
        } else {
            let cos_theta_t = (1.0 - sin_2_theta_t).sqrt();
            Some((-*wi * eta) + *n * (eta * cos_theta_i - cos_theta_t))
        }
    }
}
