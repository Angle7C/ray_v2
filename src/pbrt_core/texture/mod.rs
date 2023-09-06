use std::fmt::Debug;

use glam::f64::{DVec2, DVec3};

use super::tool::InteractionCommon;
pub mod constant;
pub mod scale;
pub mod mix;
pub mod bilinear;
pub mod mipmap;
pub mod image;
pub trait Texture<T>:Debug {
    fn  evaluate(&self,inter:&InteractionCommon)->T;
}
pub trait TextureMapping2D {
    fn  map(&self,inter:&InteractionCommon)->(DVec2,DVec2,DVec2);
}
pub trait TextureMapping3D {
    fn  map(&self,inter:&InteractionCommon)->(DVec3,DVec3,DVec3);
}