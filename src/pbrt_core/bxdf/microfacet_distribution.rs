use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use crate::pbrt_core::{tool::func::{spherical_direction, trowbridge_reitz_sample}, bxdf::func::vec3_same_hemisphere_vec3};

use super::{
    func::{ cos2_phi, cos2_theta, cos_theta, sin2_phi, sin2_theta, sin_theta},
     MicrofacetDistribution,
};

//使用Beckmann概率分布，适用各项同性
pub struct BeckmannDistribution {
    alphax: f64,
    alphay: f64,
    sample_visible_area: bool,
}
impl BeckmannDistribution {
    pub fn new(alphax: f64, alphay: f64, sample_visible_area: bool) -> Self {
        Self {
            alphax: alphax.max(0.001),
            alphay: alphay.max(0.01),
            sample_visible_area,
        }
    }
}
// 粗糙度转换
pub fn roughness_to_alpha(roughness: f64) -> f64 {
    let mut roughness = roughness;
    let limit = 1e-3;
    if limit > roughness {
        roughness = limit;
    }
    let x = roughness.ln(); // natural (base e) logarithm
    1.62142
        + 0.819_955 * x
        + 0.1734 * x * x
        + 0.017_120_1 * x * x * x
        + 0.000_640_711 * x * x * x * x
}
impl MicrofacetDistribution for BeckmannDistribution {
    fn d(&self, wh: &glam::DVec3) -> f64 {
        let tan_2_theta = sin2_theta(wh) / cos2_theta(wh);
        if tan_2_theta.is_infinite() {
            return 0.0;
        }
        let cos_4_theta = cos2_theta(wh) * cos2_theta(wh);
        (-tan_2_theta
            * (cos2_phi(wh) / (self.alphax * self.alphax)
                + sin2_theta(wh) / (self.alphay * self.alphay)))
            .exp()
            / (PI * self.alphax * self.alphay * cos_4_theta)
    }

    fn lamdba(&self, w: &glam::DVec3) -> f64 {
        let abs_tan_theta = sin_theta(w).abs() / cos_theta(w);
        if abs_tan_theta.is_infinite() {
            return 0.0;
        }
        // compute _alpha_ for direction _w_
        let alpha = (cos2_phi(w) * self.alphax * self.alphax
            + sin2_phi(w) * self.alphay * self.alphay)
            .sqrt();
        let a = 1.0 / (alpha * abs_tan_theta);
        if a >= 1.6 {
            return 0.0;
        }
        (1.0 - 1.259 * a + 0.396 * a * a) / (3.535 * a + 2.181 * a * a)
    }

    fn sample_wh(&self, _w_out: &glam::DVec3, _w_in: DVec2) -> glam::DVec3 {
        todo!()
    }

    fn pdf(&self, w_out: &glam::DVec3, wh: &glam::DVec3) -> f64 {
        if self.sample_visible_area {
            self.d(wh) * self.g1(w_out) * w_out.dot(*wh).abs() / cos_theta(w_out).abs()
        } else {
            self.d(wh) * cos_theta(wh).abs()
        }
    }
}
//GGX模型
pub struct TrowbridgeReitzDistribution {
    pub alphax: f64,
    pub alphay: f64,
    pub sample_visible_area: bool,
}
impl TrowbridgeReitzDistribution {
    pub fn new(alphax: f64, alphay: f64, sample_visible_area: bool) -> Self {
        Self {
            alphax: alphax.max(0.001),
            alphay: alphay.max(0.01),
            sample_visible_area,
        }
    }
}
impl MicrofacetDistribution for TrowbridgeReitzDistribution {
    fn d(&self, wh: &glam::DVec3) -> f64 {
        let tan_2_theta = sin2_theta(wh) / cos2_theta(wh);
        if tan_2_theta.is_infinite() {
            return 0.0;
        }
        let cos_4_theta = cos2_theta(wh) * cos2_theta(wh);
        let e=tan_2_theta
            * (cos2_phi(wh) / (self.alphax * self.alphax)
                + sin2_theta(wh) / (self.alphay * self.alphay));
        1.0/(PI*self.alphax*self.alphay*cos_4_theta*(1.00+e)*(1.0+e))
    }
    fn lamdba(&self, w: &glam::DVec3) -> f64 {
        let abs_tan_theta = sin_theta(w).abs() / cos_theta(w);
        if abs_tan_theta.is_infinite() {
            return 0.0;
        }
        // compute _alpha_ for direction _w_
        let alpha = (cos2_phi(w) * self.alphax * self.alphax
            + sin2_phi(w) * self.alphay * self.alphay)
            .sqrt();
        let a = (alpha * abs_tan_theta).powf(2.0);
       
       (-1.0+(1.0+a).sqrt())/2.0
    
    }
    fn sample_wh(&self, w_out: &glam::DVec3, u: glam::DVec2) -> glam::DVec3 {
        let mut wh:DVec3;
        if !self.sample_visible_area{
           
            let mut phi=(2.0*PI)*u.y;
            let cos_theta=if self.alphax==self.alphay{
                let tan_theta2=self.alphax*self.alphax*u.x/(1.0-u.x);
                1.0/(1.0+tan_theta2).sqrt()
            }else{
                phi=(self.alphay/self.alphax * (2.0*PI*u.y+0.5*PI).tan()).atan();
                if u.y>0.5{
                    phi+=PI;
                }
                let (sin,cos)=phi.sin_cos();
                let alphax2=self.alphax*self.alphax;
                let alphay2=self.alphay*self.alphay;
                let alpha2=1.0/(cos*cos/alphax2+sin*sin/alphay2);
                let tan_theta2=alpha2*u.x/(1.0-u.x);
                1.0/(1.0+tan_theta2).sqrt()
            };
            let sin_theta=0.0_f64.max(1.0-cos_theta*cos_theta).sqrt();
            wh=spherical_direction(sin_theta, cos_theta, phi);
            if !vec3_same_hemisphere_vec3(w_out, &wh){
                wh=-wh;
            }else{
                wh = trowbridge_reitz_sample(&-*w_out,self.alphax,self.alphay,u.x,u.y)* if w_out.z<0.0{
                    -1.0
                }else{
                    1.0
                };
            }
            wh
        }else{
            unimplemented!()
        }
    }
    fn pdf(&self, w_out: &glam::DVec3, wh: &glam::DVec3) -> f64 {
       if self.sample_visible_area{
            self.d(wh)*self.g1(w_out)*w_out.dot(*wh).abs()/cos_theta(w_out).abs()
        }else{
            self.d(wh)*cos_theta(wh).abs()
        }
    }
}
