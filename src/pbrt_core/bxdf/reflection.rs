use std::f64::consts::*;

use glam::f64::DVec3;

use crate::pbrt_core::tool::color::Color;

use super::{BxDFAble, BxDFType, func::{self, cos_theta}, MicrofacetDistribution, frensnel::Fresnel};

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
}


pub struct MicrofacetReflection{
    r:Color,
    distribution:Box<dyn MicrofacetDistribution>,
    fresnel:Fresnel,
}
impl MicrofacetReflection{
    pub fn new(_r:Color,_distribution:Box<dyn MicrofacetDistribution>,_frensnel:Fresnel){
        unimplemented!()
    }
}
impl BxDFAble for MicrofacetReflection{
    fn match_type(&self, flag: u32) -> bool {
        (BxDFType::Reflection | BxDFType::Glossy) &flag >0
    }

    fn fi(&self, w_in: &DVec3, w_out: &DVec3) -> DVec3 {
        let cos_o=cos_theta(w_out).abs();
        let cos_i=cos_theta(w_in).abs();
        let mut wh=*w_in+*w_out;
        if cos_i==0.0 || cos_o==0.0{
            return DVec3::ZERO;
        }
        if wh.abs_diff_eq(DVec3::ZERO, f64::EPSILON){
            return DVec3::ZERO;
        }
        wh=wh.normalize();
        let dot=w_in.dot(wh);
        let f=self.fresnel.evaluate(dot);
        self.r*self.distribution.d(&wh) * self.distribution.g(w_out, w_in)
        *f/(4.0 * cos_i*cos_o)

    }
    fn pdf(&self,w_out: DVec3, w_in: DVec3) -> f64 {
        if func::vec3_same_hemisphere_vec3(&w_out, &w_in){
            0.0
        }else{
            let wh=(w_in+w_out).normalize();
            self.distribution.pdf(&w_out, &wh)/(4.0*w_out.dot(wh))
        }
    }
    fn sample_f(
            &self,
            w_in: &mut DVec3,
            w_out: &DVec3,
            sample_point: glam::DVec2,
            pdf: &mut f64,
        ) -> DVec3 {
        if w_out.z==0.0{
            return Color::ZERO;
        }
        let wh=self.distribution.sample_wh(w_out, sample_point);
        *w_in=func::reflect(w_out, &wh);
        if func::vec3_same_hemisphere_vec3(w_out, &w_in){
            return Color::ZERO
        }
        *pdf=self.distribution.pdf(w_out, &wh)/(4.0*w_out.dot(wh));
        self.fi(w_in, w_out)
    }
}