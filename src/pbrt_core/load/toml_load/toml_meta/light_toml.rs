use std::sync::Arc;
use glam::{Mat4, Vec3, vec3};
use serde::{Deserialize, Serialize};
use crate::pbrt_core::light::area::DiffuseAreaLight;
use crate::pbrt_core::light::inf::InfiniteLight;
use crate::pbrt_core::light::Light;
use crate::pbrt_core::light::point::Point;
use crate::pbrt_core::load::toml_load::toml_meta::texture_toml::TextureToml;

use crate::pbrt_core::primitive::Primitive;
use crate::pbrt_core::texture::Texture;

use super::{base_toml::TransformToml};

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum LightToml {
    Point {
        point: Vec3,
        emit: Vec3,
    },
    Spot {
        trans: TransformToml,
        point: Vec3,
        emit: Vec3,
        end_angle: f32,
        start_angle: f32,
    },
    Texture {
        emit: Vec3,
        texture: TextureToml,
    },
    Distant {
        emit: Vec3,
        dir: Vec3,
        world_center: Vec3,
        world_radius: Vec3,
    },
    Area {
        emit: Vec3,
        shape_index: usize,
    },
    Infinite {
        skybox: usize,
        world_center: Vec3,
        world_radius: f32,
    },
}
impl LightToml{
    pub fn get(&self,primitive:&[Arc<dyn Primitive>],textures:&[Arc<dyn Texture>])->Arc<Light>{
         Arc::new(match *self {
            LightToml::Point { point,emit } => {
               Light::PointLight( Box::new(Point::new(emit,point,usize::MAX)))
            }

            LightToml::Area { emit,shape_index } => {
              Light::AreaLight(Box::new(DiffuseAreaLight::new(emit,primitive.get(shape_index).expect("获取图元失败").clone(),shape_index)))
            }
            LightToml::Infinite { skybox,world_center,world_radius } => {
                Light::Infinite(
                Box::new(InfiniteLight::new(world_radius,
                                   world_center,
                                   textures.get(skybox).expect("获取天空盒失败").clone(),
                                   Mat4::IDENTITY,
                                   Vec3::splat(10.0),usize::MAX)))
            }
            LightToml::Spot { .. } => {todo!()}
            LightToml::Texture { .. } => {todo!()}
            LightToml::Distant { .. } => {todo!()}
        })
    }
}