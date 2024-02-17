use std::sync::Arc;
use glam::{Mat4, Vec3};
use serde::{Deserialize, Serialize};
use crate::pbrt_core::light::area::DiffuseAreaLight;
use crate::pbrt_core::light::inf::InfiniteLight;
use crate::pbrt_core::light::{Light, LightAble};
use crate::pbrt_core::light::point::Point;

use crate::pbrt_core::texture::Texture;

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum LightToml {
    Point {
        point: Vec3,
        emit: Vec3,
    },
    Area {
        emit: Vec3,
    },
    Infinite {
        skybox: usize,
        world_center: Vec3,
        world_radius: f32,
        emit:f32
    },
}
impl LightToml{
    pub fn get(&self,textures:&[Arc<dyn Texture>])->Arc<dyn LightAble>{
         Arc::new(match *self {
            LightToml::Point { point,emit } => {
               Light::PointLight( Box::new(Point::new(emit,point,usize::MAX)))
            }

            LightToml::Area { emit} => {
              Light::AreaLight(Box::new(DiffuseAreaLight::new(emit)))
            }
            LightToml::Infinite { skybox,world_center,world_radius,emit } => {
                Light::Infinite(
                Box::new(InfiniteLight::new(world_radius,
                                   world_center,
                                   textures.get(skybox).expect("获取天空盒失败").clone(),
                                   Mat4::IDENTITY,
                                   Vec3::splat(emit),usize::MAX)))
            }
        })
    }
}