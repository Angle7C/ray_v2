use std::fmt::Debug;

use glam::Vec3;

use super::tool::InteractionCommon;
pub mod constant;
pub mod scale;
pub mod mix;
pub mod image;
pub trait Texture: Debug {
    fn  evaluate(&self,inter:&InteractionCommon)->Vec3;
}