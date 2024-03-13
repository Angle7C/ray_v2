use std::fmt::Debug;




use super::tool::{color::Color, InteractionCommon};
//常量纹理
pub mod constant;
//放缩纹理
pub mod scale;
//混合纹理
pub mod mix;
//图片纹理
pub mod image;
// 纹理数据
pub trait Texture: Debug {
    fn  evaluate(&self,inter:&InteractionCommon)->Color;
}