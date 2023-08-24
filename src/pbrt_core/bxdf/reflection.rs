use std::f64::consts::*;

use glam::f64::DVec3;

use super::{BxDFAble, BxDFType};

pub struct LambertianReflection{
    r: DVec3
}
impl BxDFAble for LambertianReflection{
    fn fi(&self, _w_in: glam::DVec3, _w_out: glam::DVec3) -> glam::DVec3 {
        self.r*FRAC_1_PI
    }
    fn match_type(&self, flag: super::BxDFType) -> bool {
        ((BxDFType::Reflection | BxDFType::Diffuse) & flag as u32) != 0

    }
    fn rho(&self, _w_in:glam::DVec3, _w_out: glam::DVec3, _sample_point: glam::DVec2) -> glam::DVec3 {
        self.r
    }
    fn sample_fi(&self, w_in: &mut glam::DVec3, w_out: glam::DVec3, _sample_point: glam::DVec2) -> glam::DVec3 {
        self.fi(*w_in, w_out)
    }
}