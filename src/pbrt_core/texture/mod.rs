use std::fmt::Debug;

use super::tool::InteractionCommon;
pub mod constant;
pub mod scale;
pub mod mix;
pub mod mipmap;
pub mod image;
pub trait Texture<T>: Debug {
    fn  evaluate(&self,inter:&InteractionCommon)->T;
}