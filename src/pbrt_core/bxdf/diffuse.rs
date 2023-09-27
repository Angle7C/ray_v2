use std::f32::consts::{FRAC_1_PI, FRAC_2_PI, TAU};

use crate::pbrt_core::tool::color::Color;

use super::{BxDFAble, BxDFType};

pub struct LambertianReflection {
    r: Color,
}
impl BxDFAble for LambertianReflection {
    fn f(&self, w0: &glam::Vec3A, wi: &glam::Vec3A) -> Color {
        self.r * FRAC_1_PI
    }
    fn get_type(&self) -> u32 {
        BxDFType::Diffuse as u32 | BxDFType::Reflection as u32
    }
    fn pdf(&self, wo: &glam::Vec3A, wi: &glam::Vec3A) -> f32 {
        1.0/TAU
    }

}
