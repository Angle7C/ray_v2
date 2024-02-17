use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::pbrt_core::primitive::shape::{rectangle::Rectangle, sphere::Sphere, ShapeAble};

use super::base_toml::TransformToml;

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum ShapeToml {
    Rect {
        transform: TransformToml,
    },
    Sphere {
        transform: TransformToml,
        r: f32,
    },
}
impl ShapeToml{

    pub fn get(&self)->Arc<dyn ShapeAble>{
        match self {
            ShapeToml::Rect{transform} => {
               Arc::new( Rectangle::new(transform.get()))
            },
            ShapeToml::Sphere { transform, r } => {
                Arc::new(Sphere::new(*r, transform.get()))
            }
        }
    }
}