use glam::{Vec2, Vec3A};

use super::tool::{
    color::Color,
    interaction::{InteractionCommon, SurfaceInteraction},
    vistest::VisibilityTester, ray::Ray,
};
pub mod area;
pub mod inf;
pub mod point;

pub trait LightAble {
    fn sample_li(
        &self,
        common: &InteractionCommon,
        light_common: &mut InteractionCommon,
        u: &Vec2,
        wi: &mut Vec3A,
        pdf: &mut f32,
        vis: &mut VisibilityTester,
    ) -> Color;
    fn pdf_li(&self, 
        // vis: &mut VisibilityTester,
         inter: &SurfaceInteraction, wi: Vec3A) -> f32;
    fn get_type(&self) -> u32;
    fn get_n_samples(&self) -> usize {
        1
    }
    fn le(&self,ray:&Ray)->Color{
        Color::ZERO
    }
}

pub enum LightType {
    DeltaPosition=1,
    DeltaDirection=2,
    Area=4,
    Infinite=8,
}
impl LightType{
    pub fn is_delta(flag:u32)->bool{
        (LightType::DeltaPosition as u32 |LightType::DeltaDirection as u32)  &flag>0
    }
}
pub enum TransportMode{
    Radiance,
    Out
}