use glam::Vec3A;

use crate::pbrt_core::tool::color::Color;

use super::{fresnel::Fresnel, BxDFAble, BxDFType, func::{self, cos_theta, cos_theta_abs}};

pub struct SpecularReflection{
    r:Color,
    fresnel:Fresnel
}
impl BxDFAble for SpecularReflection{
    fn f(&self, _w0: &glam::Vec3A, _wi:&glam::Vec3A) -> Color {
        Color::ZERO
    }
    fn get_type(&self) -> u32 {
        BxDFType::Reflection as u32 | BxDFType::Specular as u32
    }
    fn pdf(&self, _wo: &glam::Vec3A, _wi: &glam::Vec3A) -> f32 {
        0.0
    }
    fn sample_f(
            &self,
            wo: &glam::Vec3A,
            wi: &mut glam::Vec3A,
            _u_point: &glam::Vec2,
            pdf: &mut f32,
            _sampled_type: Option<&mut u32>,
        ) -> Color {
        
        *wi=func::reflect(wo, Vec3A::Z);
        *pdf=1.0;
        self.fresnel.evaluate(cos_theta(&wi))*self.r/cos_theta_abs(&wi)
    }
}