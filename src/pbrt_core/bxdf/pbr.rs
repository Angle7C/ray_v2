use std::f32::consts::FRAC_1_PI;

use glam::Vec3;

use crate::pbrt_core::tool::color::Color;

use super::{BxDFAble, func::{cos_theta, self}, BxDFType, MicrofacetDistribution, frensnel::Fresnel};

//漫反射射项
pub struct PbrDiff{
    r: Color,
}
impl PbrDiff {
    pub fn new(r: Color) -> Self {
        Self { r }
    }
}
impl BxDFAble for PbrDiff {
    fn f(&self, w_in: &Vec3, w_out: &Vec3) -> Vec3 {
        let fo = func::schlick_weight(cos_theta(w_out).abs());
        let fi = func::schlick_weight(cos_theta(w_in).abs());

        self.r * FRAC_1_PI * (1.0 - fo / 2.0) * (1.0 - fi / 2.0)
    }
    fn match_type(&self, flag: u32) -> bool {
        (BxDFType::Reflection | BxDFType::Diffuse) & flag > 0
    }
    fn get_type(&self)->u32 {
        BxDFType::Reflection | BxDFType::Diffuse
    }
}
pub struct PbrReflection{
    r:Color,
    distribution:Box<dyn MicrofacetDistribution>,
    fresnel:Fresnel,
}
impl PbrReflection{
    pub fn new(r:Color,distribution:Box<dyn MicrofacetDistribution>,fresnel:Fresnel)->Self{
        Self { r, distribution, fresnel }
    }
}
impl BxDFAble for PbrReflection{
    fn match_type(&self, flag: u32) -> bool {
        (BxDFType::Reflection | BxDFType::Glossy) &flag >0
    }

    fn f(&self, w_in: &Vec3, w_out: &Vec3) -> Vec3 {
        let cos_o=cos_theta(w_out).abs();
        let cos_i=cos_theta(w_in).abs();
        let mut wh=*w_in+*w_out;
        if cos_i==0.0 || cos_o==0.0{
            return Vec3::ZERO;
        }
        if wh.abs_diff_eq(Vec3::ZERO, f32::EPSILON){
            return Vec3::ZERO;
        }
        wh=wh.normalize();
        let dot=w_in.dot(wh);
        let f=self.fresnel.evaluate(dot);
        self.r*self.distribution.d(&wh) * self.distribution.g(w_out, w_in)
        *f/(4.0 * cos_i*cos_o)

    }
    fn pdf(&self,w_out: Vec3, w_in: Vec3) -> f32 {
        if func::vec3_same_hemisphere_vec3(&w_out, &w_in){
            0.0
        }else{
            let wh=(w_in+w_out).normalize();
            self.distribution.pdf(&w_out, &wh)/(4.0*w_out.dot(wh))
        }
    }
    fn sample_f(
            &self,
            w_in: &mut Vec3,
            w_out: &Vec3,
            sample_point: glam::Vec2,
            pdf: &mut f32,
        ) -> Vec3 {
        if w_out.z==0.0{
            return Color::ZERO;
        }
        let wh=self.distribution.sample_wh(w_out, sample_point);
        *w_in=func::reflect(w_out, &wh);
        if func::vec3_same_hemisphere_vec3(w_out, w_in){
            return Color::ZERO
        }
        *pdf=self.distribution.pdf(w_out, &wh)/(4.0*w_out.dot(wh));
        self.f(w_in, w_out)
    }
    fn get_type(&self)->u32 {
        BxDFType::Reflection | BxDFType::Glossy
    }
}