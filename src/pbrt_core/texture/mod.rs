use std::fmt::Debug;

use glam::Vec3;


use super::tool::{color::Color, InteractionCommon};
pub mod constant;
pub mod scale;
pub mod mix;
pub mod image;
pub trait Texture: Debug {
    fn  evaluate(&self,inter:&InteractionCommon)->Color;
}