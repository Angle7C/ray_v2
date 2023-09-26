// use glam::{Vec3A, Vec2, Vec4, Affine3A, Quat, Vec3};
// use serde_derive::{Serialize, Deserialize};

use std::sync::Arc;

use glam::{Affine3A, Quat, Vec2, Vec3, Vec3A, Vec4};
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    material::matte::Matte,
    primitives::shape::{rect::Rectangle, shpere::Shpere}, light::area::DiffuseAreaLight,
};

use super::{
    light::{LightAble, point::PointLight},
    material::{mirror::Mirror, MaterialAble},
    primitives::shape::ShapeAble,
    texture::{constant::ConstantTexture, Texture}, integrator::LightStartegy,
};

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct TomlLoad {
    camera: CameraToml,
    primitive: Vec<ShapeToml>,
    shapes: Vec<ShapeToml>,
    material: Vec<MaterialToml>,
    light: Vec<LightToml>,
    // env_light: Vec<LightToml>,
    texture: Vec<TextureToml>,
    pub intergator: IntegratorToml,
    name: String,
}

// impl MyLoad {

// }

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct CameraToml {
    pub mode: String,
    pub size: Vec2,
    pub far: f32,
    pub near: f32,
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct TransformToml {
    r: Vec4,
    s: Vec3A,
    t: Vec3A,
}

impl TransformToml {
    pub fn new(&self) -> Affine3A {
        let angle = self.r.w.to_radians();
        let quat = Quat::from_axis_angle(self.r.truncate(), angle);
        Affine3A::from_scale_rotation_translation(self.s.into(), quat.into(), self.t.into())
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum ShapeToml {
    Rect {
        trans: TransformToml,
        material_index: usize,
    },
    Shpere {
        trans: TransformToml,
        r: f32,
        material_index: usize,
    },
}
impl ShapeToml {
    pub fn new<'a, 'b>(self, material: &'a [Box<dyn MaterialAble + 'a>]) -> Box<dyn ShapeAble + 'b>
    where
        'a: 'b,
    {
        match self {
            ShapeToml::Rect {
                trans,
                material_index,
            } => {
                let material = material[material_index].as_ref();
                Box::new(Rectangle::new(trans.new(), Some(material)))
            }
            ShapeToml::Shpere {
                trans,
                r,
                material_index,
            } => {
                let material = material[material_index].as_ref();
                Box::new(Shpere::new(trans.new(), r, Some(material)))
            }
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum MaterialToml {
    Matte {
        kd: usize,
        sigma: f32,
    },
    Plastic {
        kd: usize,
        ks: usize,
        roughness: usize,
    },
    Mirror {
        kr: usize,
    },
}
impl MaterialToml {
    pub fn new(&self, texture: Vec<Arc<dyn Texture>>) -> Box<dyn MaterialAble> {
        match &self {
            MaterialToml::Matte { kd, sigma } => {
                let kd = texture[*kd].clone();
                Box::new(Matte::new(kd, *sigma))
            }
            MaterialToml::Mirror { kr } => {
                let kr = texture[*kr].clone();
                Box::new(Mirror::new(kr))
            }
            _ => todo!(),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum TextureToml {
    Image { path: String },
    Constant { value: Vec3A },
}
impl TextureToml {
    pub fn new(self) -> Arc<dyn Texture> {
        match self {
            TextureToml::Image { path } => todo!(),
            TextureToml::Constant { value } => Arc::new(ConstantTexture::new(value)),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum LightToml {
    Point {
        // trans: TransformToml,
        point: Vec3A,
        lemit: Vec3A,
    },
    Spot {
        trans: TransformToml,
        point: Vec3A,
        lemit: Vec3A,
        end_angle: f32,
        start_angle: f32,
    },
    Texture {
        lemit: Vec3A,
        texture: TextureToml,
    },
    Distant {
        lemit: Vec3A,
        dir: Vec3A,
        world_center: Vec3A,
        world_radius: Vec3A,
    },
    Area {
        lemit: Vec3A,
        shape_index: usize,
    },
    Infinite {
        skybox: usize,
        lemit: Vec3A,
        world_center: Vec3A,
        world_radius: f32,
    },
}
impl LightToml {
    pub fn new<'a, 'b>(self, shape: &'b [Box<dyn ShapeAble>]) -> Box<dyn LightAble + 'b>
    where
        'a: 'b,
    {
        match self{
            LightToml::Area { lemit, shape_index } =>{
                let shape=shape[shape_index].as_ref();
                Box::new(DiffuseAreaLight::new(lemit, shape))
            },
            LightToml::Point { point, lemit } => {
                Box::new(PointLight::new(point,lemit.into()))
            },
            LightToml::Infinite { skybox, lemit, world_center, world_radius } => todo!(),
            _=>todo!()
         
      
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[serde(tag = "mode")]
pub enum IntegratorToml {
    Path {
        core_num: usize,
        sample_num: usize,
        q: f32,
        max_depth: usize,
    },
    Direct {
        core_num: usize,
        sample_num: usize,
        startegy: LightStartegy,
    },
}

impl Default for IntegratorToml {
    fn default() -> Self {
        Self::Path {
            core_num: 8,
            sample_num: 1,
            q: 0.9,
            max_depth: 5,
        }
    }
}
