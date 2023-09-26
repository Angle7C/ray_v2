use glam::{Vec2, Vec3A};

use super::tool::{
    color::Color,
    interaction::{InteractionCommon, SurfaceInteraction},
    vistest::VisibilityTester,
};
pub mod area;
pub mod inf;
pub mod point;

pub trait LightAble {
    fn sample_li(
        &self,
        common: &InteractionCommon,
        light_common: &mut InteractionCommon,
        u: Vec2,
        wi: &mut Vec3A,
        pdf: &mut f32,
        vis: &mut VisibilityTester,
    ) -> Color;
    fn pdf_li(&self, 
        // vis: &mut VisibilityTester,
         inter: &SurfaceInteraction, wi: Vec3A) -> f32;
    fn get_type(&self) -> LightType;
    fn get_n_samples(&self) -> usize {
        1
    }
}

pub enum LightType {
    DeltaPosition,
    DeltaDirection,
    Area,
    Infinite,
}
pub enum TransportMode{
    Radiance,
    Out
}