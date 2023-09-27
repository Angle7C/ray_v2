use crate::pbrt_core::tool::color::Color;

pub enum Fresnel{
    Conductor(FresnelConductor),
    Dielectric(FresnelDielectric),
    NoOp
}
impl Fresnel{
    pub fn evaluate(&self,cos_i:f32)->Color{
        match &self{
           Self::Conductor(_FresnelConductor)=>unimplemented!("conductor evaluate"),
           Self:: Dielectric(_FresnelDielectric)=>unimplemented!("dielectric evaluate"),
           Self::NoOp=>Color::ONE
        }
    }
}

pub struct FresnelConductor{
    eta_i:Color,
    eta_t:Color,
    k:Color
}
pub struct FresnelDielectric{
    eta_i:f32,
    eta_t:f32,
}

