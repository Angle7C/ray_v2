
use super::tool::{interaction::SurfaceInteraction, color::Color};
pub mod constant;
pub trait  Texture {
    fn evaluate(&self,common:&SurfaceInteraction)->Color;
}