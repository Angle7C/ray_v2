use std::f32::consts::FRAC_1_PI;

use glam::{Vec2,Vec3};

use self::{reflection::{LambertianReflection, OrenNayar}, specular::SpecularReflection, pbr::{PbrDiff, PbrReflection}};

use super::sampler::cosine_sample_hemisphere;
// 菲涅尔反射率
pub mod frensnel;
// 高光
pub mod specular;
//反射
pub mod reflection;
//溦表面模型
pub mod microfacet;
//
pub mod microfacet_distribution;

pub mod pbr;

pub mod disney;
//基本模型
pub trait BxDFAble {
    //匹配BxDF类型
    fn match_type(&self, flag: u32) -> bool;
    //计算，从wi射入，到wo射出时，光线被反射了多少回去[Vec3::ZERO,Vec3::ONE]
    fn fi(&self, w_in: &Vec3, w_out: &Vec3) -> Vec3;
    //根据采样点sample_point计算，从wi射入，到wo射出时，的双向分布函数值。
    fn sample_f(
        &self,
        w_in: &mut Vec3,
        w_out: &Vec3,
        sample_point: Vec2,
        pdf: &mut f32,
    ) -> Vec3 {
        *w_in = cosine_sample_hemisphere(sample_point).normalize();
        if w_out.z < 0.0 {
            w_in.z *= -1.0
        }
        *pdf = self.pdf(*w_out, *w_in);
        return self.fi(w_in, &w_out);
    }
    fn pdf(&self,w_out: Vec3, w_in: Vec3) -> f32 {
        if w_out.z * w_in.z > 0.0 {
            w_in.z * FRAC_1_PI
        } else {
            0.0
        }
    }
}


// 溦表面模型
pub trait MicrofacetDistribution {
    //给定法向量，求得微分面积
    fn d(&self,wh:&Vec3)->f32;
    //
    fn lamdba(&self,w:&Vec3)->f32;
    //法线分布函数
    fn g1(&self,w:&Vec3)->f32{
        1.0/(1.0+self.lamdba(w))
    }
    fn g(&self,w_out:&Vec3,w_in:&Vec3)->f32{
        1.0/(1.0+self.lamdba(w_out)+self.lamdba(w_in))
    }
    fn sample_wh(&self,w_out:&Vec3,u:Vec2)->Vec3;
    fn pdf(&self,w_out:&Vec3,wh:&Vec3)->f32;

}
pub enum BxDF {
    LambertianReflection(LambertianReflection),
    SpecularReflection(SpecularReflection),
    OrenNayar(OrenNayar),
    PbrDiff(PbrDiff),
    PbrReflection(PbrReflection)
}
impl BxDF {
    pub fn match_type(&self, flag: u32) -> bool {
        match &self {
            Self::LambertianReflection(lambert) => lambert.match_type(flag),
            Self::SpecularReflection(spec_ref)=>spec_ref.match_type(flag),
            Self::OrenNayar(oren)=>oren.match_type(flag),
            Self::PbrDiff(diff)=>diff.match_type(flag),
            Self::PbrReflection(reflection)=>reflection.match_type(flag)
        }
    }
    pub fn f(&self, w_out: &Vec3, w_in: &mut Vec3) -> Vec3 {
        match &self {
            BxDF::LambertianReflection(lam) => lam.fi(w_in, w_out),
            BxDF::SpecularReflection(spec_ref)=>spec_ref.fi(w_in, w_out),
            Self::OrenNayar(oren)=>oren.fi(w_in, w_out),
            Self::PbrDiff(diff)=>diff.fi(w_in, w_out),
            Self::PbrReflection(reflection)=>reflection.fi(w_in, w_out)
        }
    }
    pub fn sample_f(&self, w_out: &Vec3, wi: &mut Vec3, u: Vec2, pdf: &mut f32) -> Vec3 {
        match &self {
            Self::LambertianReflection(lambert) => lambert.sample_f(wi, w_out, u, pdf),
            BxDF::SpecularReflection(spec_ref)=>spec_ref.sample_f(wi, w_out, u, pdf),
            Self::OrenNayar(oren)=>oren.sample_f(wi, w_out, u, pdf),
            Self::PbrDiff(diff)=>diff.sample_f(wi, w_out, u, pdf),
            Self::PbrReflection(reflection)=>reflection.sample_f(wi, w_out, u, pdf),
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
#[derive(Debug, Clone, Copy)]
pub enum TransportMode {
    Radiance,
    Importance,
}
impl PartialEq<TransportMode> for TransportMode {
    fn eq(&self, other: &TransportMode) -> bool {
        match (self, other) {
            (TransportMode::Radiance, TransportMode::Radiance) => true,
            (TransportMode::Importance, TransportMode::Importance) => true,
            _ => false,
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

pub(crate) mod func {
    use glam::Vec3;
    //计算菲涅尔反射率，介电材料
    //eta_i 入射折射率
    //eta_t 出射折射率
    //cos_theta_i: 入射角
    #[allow(unused)]
    #[inline]
    pub fn fr_dielectric(cos_theta_i: f32, eta_i: f32, eta_t: f32) -> f32 {
        let mut cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
        let entering = cos_theta_i > 0.0;
        let mut local_eta_i = eta_i;
        let mut local_eta_t = eta_t;
        if !entering {
            std::mem::swap(&mut local_eta_i, &mut local_eta_t);
            cos_theta_i = cos_theta_i.abs();
        }
        let sin_theta_i = 0.0f32.max(1.0 - cos_theta_i * cos_theta_i).sqrt();
        let sin_theta_t = local_eta_i / local_eta_t * sin_theta_i;
        if sin_theta_t >= 1.0 {
            return 1.0;
        }
        let cos_theta_t = 0.0f32.max(1.0 - sin_theta_t * sin_theta_t).sqrt();
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
    #[inline]
    pub fn fr_conductor(cos_theta_i: f32, eta_i: Vec3, eta_t: Vec3, k: Vec3) -> Vec3 {
        let not_clamped = cos_theta_i;
        let cos_theta_i = not_clamped.clamp(-1.0, 1.0);
        let eta = eta_t / eta_i;
        let eta_k = k / eta_i;
        let cos_theta_i2 = cos_theta_i * cos_theta_i;
        let sin_theta_i2 = 1.0 - cos_theta_i2;
        let eta_2 = eta * eta;
        let eta_k2 = eta_k * eta_k;
        let t0 = eta_2 - eta_k2 - Vec3::splat(sin_theta_i2);
        let a2_plus_b2 = (t0 * t0 + eta_2 * eta_k2 * Vec3::splat(4.0)).powf(0.5);
        let t1 = a2_plus_b2 + Vec3::splat(cos_theta_i2);
        let a = ((a2_plus_b2 + t0) * 0.5).powf(0.5);
        let t2 = a * 2.0 * cos_theta_i;
        let rs = (t1 - t2) / (t1 + t2);
        let t3 = a2_plus_b2 * cos_theta_i2 + Vec3::splat(sin_theta_i2 * sin_theta_i2);
        let t4 = t2 * sin_theta_i2;
        let rp = rs * (t3 - t4) / (t3 + t4);
        (rp + rs) * Vec3::splat(0.5)
    }
    #[inline]
    pub fn pow5(v: f32) -> f32 {
        (v * v) * (v * v) * v
    }
    #[allow(unused)]
    #[inline]
    pub fn schlick_weight(cos_theta: f32) -> f32 {
        let m = (1.0 - cos_theta).clamp(0.0, 1.0);
        pow5(m)
    }
    #[allow(unused)]
    #[inline]
    pub fn fr_schlick(r0: f32, cos_theta: f32) -> f32 {
        schlick_weight(cos_theta)
    }
    #[allow(unused)]

    pub fn fr_schlick_spectrum(r0: Vec3, cos_theta: f32) -> Vec3 {
        let r = schlick_weight(cos_theta);
        Vec3::lerp(r0, Vec3::ONE, r)
    }
    #[allow(unused)]
    pub fn refract(wi: &Vec3, n: &Vec3, eta: f32, wt: &mut Vec3) -> bool {
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
    #[inline]
    pub fn _face_forward(normal: Vec3, wo: Vec3) -> Vec3 {
        if normal.dot(wo) > 0.0 {
            normal
        } else {
            wo
        }
    }
    #[inline]
    pub fn sin_theta(wi: &Vec3) -> f32 {
        sin2_theta(wi).sqrt()
    }
    #[inline]
    pub fn sin2_theta(wi: &Vec3) -> f32 {
        (1.0 - wi.z).max(0.0)
    }
    #[inline]
    pub fn cos_theta(wi: &Vec3) -> f32 {
      wi.z
    }
    pub fn cos2_theta(wi: &Vec3)->f32{
        cos_theta(wi)*cos_theta(wi)
    }
    pub fn cos_phi(w: &Vec3) -> f32 {
        let sin_theta = sin_theta(w);
        if sin_theta == 0.0  {
            1.0
        } else {
            (w.x / sin_theta).clamp(-1.0, 1.0)
        }
    }
    pub fn cos2_phi(w:&Vec3)->f32{
        cos_phi(w)*cos_phi(w)
    }
    pub fn sin_phi(w: &Vec3) -> f32 {
        let sin_theta = sin_theta(w);
        if sin_theta == 0.0  {
            1.0
        } else {
            (w.x / sin_theta).clamp(-1.0, 1.0)
        }
    }
    pub fn sin2_phi(w: &Vec3) -> f32 {
       sin_phi(w)*sin_phi(w)
    }
    pub fn vec3_same_hemisphere_vec3(w: &Vec3, wp: &Vec3) -> bool {
        w.z * wp.z > 0.0 
    }
    pub fn reflect(wo: &Vec3, n: &Vec3) -> Vec3 {
        -(*wo) + *n * 2.0 * wo.dot(*n)
    }
}
