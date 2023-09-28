// use glam::{Vec3A, Vec2, Vec4, Affine3A, Quat, Vec3};
// use serde_derive::{Serialize, Deserialize};

use std::sync::Arc;

use glam::{Affine3A, Quat, Vec2, Vec3, Vec3A, Vec4};
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    light::area::DiffuseAreaLight,
    material::matte::Matte,
    primitives::shape::{rect::Rectangle, shpere::Shpere},
};

use super::{
    camera::{Camera, CameraMode},
    integrator::{path::PathIntegrator, Integrator, LightStartegy, SampleIntegrator},
    light::{point::PointLight, LightAble},
    material::{mirror::Mirror, MaterialAble},
    primitives::{bvh::Accel, shape::ShapeAble, GeometricPrimitive},
    texture::{constant::ConstantTexture, Texture},
    tool::{content::Setting, sence::Sence},
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
impl TomlLoad {
    pub fn build(self, sence: &mut Sence) {
        let texture: Vec<Arc<dyn Texture>> = self.load_texture();

        for material in self.material.into_iter() {
            sence.material.push(material.new(texture.as_slice()))
        }

        for (index, primitive) in self.primitive.into_iter().enumerate() {
            sence.primitive.push(primitive.new(&sence.material, index));
        }

        for light in self.light.into_iter() {
            sence.light.push(light.new(&sence.primitive));
        }
        let geometry = sence
            .primitive
            .iter()
            .map(|item| GeometricPrimitive::new(item.clone()))
            .map(|mut item| {
                let light = sence.light.iter().find_map(|lgiht| {
                    if lgiht.get_index() == item.shape.get_index() {
                        Some(lgiht)
                    } else {
                        None
                    }
                });
                item.light = if let Some(light) = light {
                    Some(light.clone())
                } else {
                    None
                };
                item
            })
            .collect::<Vec<_>>();
        sence.accel = Some(Box::new(Accel::new(geometry)));
        sence.camera = self.camera.new()
    }
    fn load_texture<'a, 'b>(&'a self) -> Vec<Arc<dyn Texture + 'b>>
    where
        'b: 'a,
    {
        let mut textures = vec![];
        for texture in self.texture.iter() {
            textures.push(texture.new())
        }
        textures
    }
    fn load_camera(self) -> Camera {
        unimplemented!()
    }
    // fn load_light<'a>(
    //     &'a self,
    //     shape: &'a [Box<dyn ShapeAble + 'a>],
    // ) -> Vec<Box<dyn LightAble + 'a>> {
    //     let mut lights = vec![];
    //     for light in self.light.into_iter() {
    //         lights.push(light.new(shape));
    //     }
    //     lights
    // }
    pub fn crate_setting(&self) -> Setting {
        let (core_num, sampler_num) = match self.intergator {
            IntegratorToml::Path {
                core_num,
                sample_num,
                ..
            } => (core_num, sample_num),
            IntegratorToml::Direct {
                core_num,
                sample_num,
                ..
            } => (core_num, sample_num),
        };
        let size=self.camera.size.as_uvec2();
        Setting {
            name: self.name.to_owned(),
            num: sampler_num as u32,
            core_num: core_num as u32,
            size
        }
    }
    pub fn crate_integrator(&self) -> SampleIntegrator {
        match self.intergator {
            IntegratorToml::Path {
                core_num,
                sample_num,
                q,
                max_depth,
            } => SampleIntegrator::new(PathIntegrator::new(q, max_depth)),
            IntegratorToml::Direct {
                core_num,
                sample_num,
                ..
            } => todo!(),
        }
    }
}

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
impl CameraToml {
    pub fn new(&self) -> Camera {
        Camera::new(
            self.eye,
            self.target,
            self.up,
            self.size,
            if self.mode == "P" {
                CameraMode::P
            } else {
                CameraMode::O
            },
            self.fov,
        )
    }
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
    pub fn new(&self, material: &[Arc<dyn MaterialAble>], inedx: usize) -> Arc<dyn ShapeAble> {
        match self {
            ShapeToml::Rect {
                trans,
                material_index,
            } => {
                let material = material[*material_index].clone();
                Arc::new(Rectangle::new(trans.new(), Some(material), inedx))
            }
            ShapeToml::Shpere {
                trans,
                r,
                material_index,
            } => {
                let material = material[*material_index].clone();
                Arc::new(Shpere::new(trans.new(), *r, Some(material), inedx))
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
    pub fn new(&self, texture: &[Arc<dyn Texture>]) -> Arc<dyn MaterialAble> {
        match &self {
            MaterialToml::Matte { kd, sigma } => {
                let kd = texture[*kd].clone();
                Arc::new(Matte::new(kd.clone(), *sigma))
            }
            MaterialToml::Mirror { kr } => {
                let kr = texture[*kr].clone();
                Arc::new(Mirror::new(kr.clone()))
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
    pub fn new(&self) -> Arc<dyn Texture> {
        match self {
            TextureToml::Image { path } => todo!(),
            TextureToml::Constant { value } => Arc::new(ConstantTexture::new(*value)),
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
    pub fn new(&self, shape: &[Arc<dyn ShapeAble>]) -> Arc<dyn LightAble> {
        match self {
            LightToml::Area { lemit, shape_index } => {
                let shape = shape[*shape_index].clone();
                Arc::new(DiffuseAreaLight::new(*lemit, shape, *shape_index))
            }
            LightToml::Point { point, lemit } => Arc::new(PointLight::new(*point, (*lemit).into())),
            LightToml::Infinite {
                skybox,
                lemit,
                world_center,
                world_radius,
            } => todo!(),
            _ => todo!(),
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
