use glam::Vec3;
use serde::{Deserialize, Serialize};
use self::base_toml::TransformToml;


pub mod base_toml;

pub mod material_toml;

pub mod light_toml;

pub mod texture_toml;

pub mod primitive_toml;


#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum ShapeToml {
    Rect {
        trans: TransformToml,
        material_index: usize,
    },
    Sphere {
        trans: TransformToml,
        r: f32,
        material_index: usize,
    },
}



