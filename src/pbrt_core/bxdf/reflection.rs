use std::f64::consts::*;

use glam::f64::DVec3;

use super::{BxDFAble, BxDFType, func};

pub struct LambertianReflection{
    r: DVec3
}
impl BxDFAble for LambertianReflection{
    fn fi(&self, _w_in: &glam::DVec3, _w_out: &glam::DVec3) -> glam::DVec3 {
        self.r*FRAC_1_PI
    }
    fn match_type(&self, flag: u32) -> bool {
        ((BxDFType::Reflection | BxDFType::Diffuse) & flag as u32) != 0

    }
    fn rho(&self, _w_in:glam::DVec3, _w_out: glam::DVec3, _sample_point: glam::DVec2) -> glam::DVec3 {
        self.r
    }
    
}
impl LambertianReflection{
    pub fn new(r:DVec3)->Self{
        Self { r }
    }
}

pub struct OrenNayar {
    r: DVec3,
    a: f64,
    b: f64,
}
impl OrenNayar {
    pub fn new(r: DVec3,sigma:f64) -> Self {
        let sigma=sigma.to_radians();
        let sigma_2=sigma*sigma;
        let a=1.0-(sigma_2/(2.0*sigma_2+0.33));
        let b= 0.45*sigma_2/(sigma_2+0.009);
        Self { r, a, b }
    }
}
impl BxDFAble for OrenNayar{
    fn match_type(&self, flag: u32) -> bool {
        ((BxDFType::Reflection | BxDFType::Diffuse) & flag as u32) != 0
    }
    #[inline]
    fn fi(&self, w_in: &DVec3, w_out: &DVec3) -> DVec3 {
        let sin_i=func::sin_theta(w_in);
        let sin_o=func::sin_theta(w_out);
        let mut max_cos:f64=0.0;
        let cos_i=func::cos_theta(w_in);
        let cos_o=func::cos_theta(w_out);
        if sin_i >f64::EPSILON && sin_o >f64::EPSILON{
        
            let d_cos=cos_i*cos_o+sin_i*sin_o;
            max_cos=d_cos.clamp(0.0, 1.0);
        }
        let (sin_alpha,tan_beta)=if cos_i.abs()>cos_o.abs(){
            (sin_o,sin_i/cos_i.abs())
        }else{
            (sin_i,sin_o/cos_o.abs())
        };
        self.r-FRAC_1_PI*(self.a+self.b*max_cos*sin_alpha*tan_beta)
    }

    fn rho(&self, w_in:DVec3, w_out: DVec3, sample_point: glam::DVec2) -> DVec3 {
        todo!()
    }
}