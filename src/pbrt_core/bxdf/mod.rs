use std::f64::consts::FRAC_1_PI;

use glam::f64::{DVec2, DVec3};

use self::reflection::LambertianReflection;

use super::sampler::cosine_sample_hemisphere;
// 菲涅尔反射率
pub mod frensnel;
// 高光
pub mod specular;
//反射
pub mod reflection;
pub trait BxDFAble {
    //匹配BxDF类型
    fn match_type(&self, flag: u32) -> bool;
    //计算，从wi射入，到wo射出时，光线被反射了多少回去[Vec3::ZERO,Vec3::ONE]
    fn fi(&self, w_in: &DVec3, w_out: &DVec3) -> DVec3;
    //根据采样点sample_point计算，从wi射入，到wo射出时，的双向分布函数值。
    fn sample_f(&self, w_in: &mut DVec3, w_out: &DVec3, sample_point: DVec2,pdf:&mut f64) -> DVec3{
        *w_in=cosine_sample_hemisphere(sample_point);
        if w_out.z<0.0{w_in.z*=-1.0}
        *pdf=Self::pdf(*w_out,*w_in);
        return self.fi(w_in, &w_out)
    }
    fn pdf(w_out:DVec3,w_in:DVec3)->f64{
        if w_out.z*w_in.z>0.0 { w_in.z *FRAC_1_PI} else{0.0}
    }
    //根据采样点sample_point计算，从wi射入，到wo射出时的反射率
    fn rho(&self, w_in:DVec3, w_out: DVec3, sample_point: DVec2) -> DVec3;
}
pub enum BxDF {
    LambertianReflection(LambertianReflection)
}
impl BxDF{
    pub fn match_type(&self,flag:u32)->bool{
        match &self {
            Self::LambertianReflection(lambert)=>{
                lambert.match_type(flag)
            }
        }
    }
    pub fn f(&self,w_out:&DVec3,w_in:&mut DVec3)->DVec3{
        match &self{
            BxDF::LambertianReflection(lam) => lam.fi(w_in, w_out),
        }
    }
    pub fn sample_f(&self,w_out:&DVec3,wi:&mut DVec3,u:DVec2,pdf:&mut f64)->DVec3{
        match &self {
            Self::LambertianReflection(lambert)=>{
                lambert.sample_f(wi,w_out, u, pdf)
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum BxDFType {
    None = 0,
    Reflection = 1,
    Transmission = 2,
    Diffuse = 4,
    Glossy = 8,
    Specular = 16,
    All = 31,
}
#[derive(Debug,Clone, Copy)]
pub enum TransportMode {
    Radiance,
    Importance,
}
impl PartialEq<TransportMode> for TransportMode{
    fn eq(&self, other: &TransportMode) -> bool {
        match (self,other){
            (TransportMode::Radiance, TransportMode::Radiance) => true,
            (TransportMode::Importance, TransportMode::Importance) => true,
            _=>false
        }
    }
}
impl std::ops::BitAnd<BxDFType> for BxDFType {
    type Output = u32;
    fn bitand(self, rhs: BxDFType) -> Self::Output {
        self as u32 & rhs as u32
    }
}
impl std::ops::BitOr<BxDFType> for BxDFType {
    type Output = u32;
    fn bitor(self, rhs: BxDFType) -> Self::Output {
        self as u32 | rhs as u32
    }
}
impl From<BxDFType> for u32 {
    fn from(value: BxDFType) -> Self {
        value as u32
    }
}



mod func {
    use glam::DVec3;
    //计算菲涅尔反射率，介电材料
    //eta_i 入射折射率
    //eta_t 出射折射率
    //cos_theta_i: 入射角
    #[allow(unused)]
    pub fn fr_dielectric(cos_theta_i: f64, eta_i: f64, eta_t: f64) -> f64 {
        let mut cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
        let entering = cos_theta_i > 0.0;
        let mut local_eta_i = eta_i;
        let mut local_eta_t = eta_t;
        if !entering {
            std::mem::swap(&mut local_eta_i, &mut local_eta_t);
            cos_theta_i = cos_theta_i.abs();
        }
        let sin_theta_i = 0.0f64.max(1.0 - cos_theta_i * cos_theta_i).sqrt();
        let sin_theta_t = local_eta_i / local_eta_t * sin_theta_i;
        if sin_theta_t >= 1.0 {
            return 1.0;
        }
        let cos_theta_t = 0.0f64.max(1.0 - sin_theta_t * sin_theta_t).sqrt();
        let r_parl = ((local_eta_t * cos_theta_i) - (local_eta_i * cos_theta_t))
            / ((local_eta_t * cos_theta_i) + (local_eta_i * cos_theta_t));
        let r_perp = ((local_eta_i * cos_theta_i) - (local_eta_t * cos_theta_t))
            / ((local_eta_i * cos_theta_i) + (local_eta_t * cos_theta_t));
        (r_parl * r_parl + r_perp * r_perp) / 2.0
    }
    //计算菲涅尔反射率，金属和电介材料
    //eta_i 入射折射率
    //eta_t 出射折射率
    //cos_theta_i: 入射角
    //k: 吸收系数
    #[allow(unused)]
    pub fn fr_conductor(cos_theta_i: f64, eta_i: DVec3, eta_t: DVec3, k: DVec3) -> DVec3 {
        let not_clamped = cos_theta_i;
        let cos_theta_i = not_clamped.clamp(-1.0, 1.0);
        let eta = eta_t / eta_i;
        let eta_k = k / eta_i;
        let cos_theta_i2 = cos_theta_i * cos_theta_i;
        let sin_theta_i2 = 1.0 - cos_theta_i2;
        let eta_2 = eta * eta;
        let eta_k2 = eta_k * eta_k;
        let t0 = eta_2 - eta_k2 - DVec3::splat(sin_theta_i2);
        let a2_plus_b2 = (t0 * t0 + eta_2 * eta_k2 * DVec3::splat(4.0)).powf(0.5);
        let t1 = a2_plus_b2 + DVec3::splat(cos_theta_i2);
        let a = ((a2_plus_b2 + t0) * 0.5).powf(0.5);
        let t2 = a * 2.0 * cos_theta_i;
        let rs = (t1 - t2) / (t1 + t2);
        let t3 = a2_plus_b2 * cos_theta_i2 + DVec3::splat(sin_theta_i2 * sin_theta_i2);
        let t4 = t2 * sin_theta_i2;
        let rp = rs * (t3 - t4) / (t3 + t4);
        (rp + rs) * DVec3::splat(0.5)
    }
    #[inline]
    pub fn pow5(v: f64) -> f64 {
        (v * v) * (v * v) * v
    }
    #[allow(unused)]

    pub fn schlick_weight(cos_theta: f64) -> f64 {
        let m = (1.0 - cos_theta).clamp(0.0, 1.0);
        pow5(m)
    }
    #[allow(unused)]

    pub fn fr_schlick(r0: f64, cos_theta: f64) -> f64 {
        schlick_weight(cos_theta)
    }
    #[allow(unused)]

    pub fn fr_schlick_spectrum(r0: DVec3, cos_theta: f64) -> DVec3 {
        let r = schlick_weight(cos_theta);
        DVec3::lerp(r0, DVec3::ONE, r)
    }
    #[allow(unused)]
    pub fn refract(wi: &DVec3, n: &DVec3, eta: f64, wt: &mut DVec3) -> bool {
        let cos_theta_i = n.dot(*wi);
        let sin2_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0);
        let sin2_theta_t = eta * eta * sin2_theta_i;
        if sin2_theta_t >= 1.0 {
            return false;
        }
        let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
        *wt = (-(*wi) * eta) + *n * (eta * cos_theta_i - cos_theta_t);
        true
    }
    pub fn face_forward(normal:DVec3,wo:DVec3)->DVec3{
        if normal.dot(wo)>0.0{
            normal
        }else {
            wo
        }
    }
}
