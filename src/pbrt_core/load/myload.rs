use std::{f32::consts::PI, sync::Arc};

use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    camera::{Camera, CameraMode},
    integrator::{
        direct::{DirectIntegrator, LightStartegy},
        path::PathIntegrator,
        Integrator,
    },
    light::{point::Point, Light},
    material::{matte::Matte, plastic::Plastic, Material},
    primitive::{
        shape::{rectangle::Rectangle, shpere::Shpere},
        Primitive,
    },
    sampler::Sampler,
    texture::{constant::ConstantTexture, image::ImageTexture, Texture},
    tool::{
        mipmap::{ImageData, MipMap},
        sence::Sence,
        setting::Setting,
    },
};
#[derive(Deserialize, Debug, Serialize, Default)]
pub struct MyLoad {
    camera: CameraToml,
    shapes: Vec<ShapeToml>,
    material: Vec<MaterialToml>,
    light: Vec<LightToml>,
    texture: Vec<TextureToml>,
    pub intergator: IntegratorToml,
    name: String,
}
impl MyLoad {
    pub fn load_sence(&self) -> Sence {
        let texture = self.load_texture().leak();
        let material = self.load_material(texture).leak();
        let shape = self.load_shape(material);
        let lights = self.load_light();
        let camera = self.load_camera();
        let sence = Sence::new(shape, camera, lights);
        sence
    }
    fn load_material(&self, texture: &'static [Arc<dyn Texture>]) -> Vec<Box<dyn Material>> {
        let mut vec = vec![];
        for material in &self.material {
            let a: Box<dyn Material> = match material {
                MaterialToml::Matte { kd, sigma } => {
                    let kd = texture.get(*kd);
                    // let sigma = texture.get(*sigma);
                    Box::new(Matte::new(kd.unwrap().clone(), *sigma))
                }
                MaterialToml::Plastic { kd, ks, roughness } => {
                    let kd = texture.get(*kd).unwrap();
                    let ks = texture.get(*ks).unwrap();
                    let roughness = texture.get(*roughness).unwrap();
                    Box::new(Plastic::new(kd.clone(), ks.clone(), roughness.clone()))
                }
                //    MaterialToml::Fourier {  } => todo!(),
                _ => todo!(),
            };
            vec.push(a)
        }
        vec
    }
    fn load_texture(&self) -> Vec<Arc<dyn Texture>> {
        let mut vec = vec![];
        for texture in &self.texture {
            let box_texture: Arc<dyn Texture> = match texture {
                TextureToml::Image { path } => {
                    let image = image::io::Reader::open(path).expect("").decode().expect("");
                    let image_data = ImageData::new_dynimage(image);
                    let mipmap = MipMap::new(image_data);
                    Arc::new(ImageTexture::new(mipmap))
                }
                TextureToml::Constant { value } => Arc::new(ConstantTexture::new(*value)),
            };
            vec.push(box_texture);
        }
        vec
    }
    fn load_shape(&self, material: &'static [Box<dyn Material>]) -> Vec<Box<dyn Primitive>> {
        let mut vec = vec![];
        for item in &self.shapes {
            let shape: Box<dyn Primitive> = match item {
                ShapeToml::Rect {
                    trans,
                    material_index,
                } => Box::new(Rectangle::new(
                    trans.get_mat(),
                    Some(material.get(*material_index).unwrap().as_ref()),
                )),
                ShapeToml::Shpere {
                    trans,
                    r,
                    material_index,
                } => Box::new(Shpere::new(
                    *r,
                    Some(material.get(*material_index).unwrap().as_ref()),
                    trans.get_mat(),
                )),
            };
            vec.push(shape)
        }
        vec
    }
    fn load_light(&self) -> Vec<Light> {
        let mut vec = vec![];
        for item in &self.light {
            let light: Light = match item {
                LightToml::Point {
                    trans,
                    point,
                    lemit,
                } => Light::PointLight(Box::new(Point::new(*lemit, *point, trans.get_mat()))),
                _ => todo!(),
            };
            vec.push(light)
        }
        vec
    }
    fn _load_env(&mut self) {}
    fn load_camera(&self) -> Camera {
        let mode = self.camera.mode.as_str();
        match mode {
            "P" => Camera::new(
                self.camera.eye,
                self.camera.target,
                self.camera.up,
                self.camera.size,
                CameraMode::P,
                self.camera.fov,
            ),
            "O" => Camera::new(
                self.camera.eye,
                self.camera.target,
                self.camera.up,
                self.camera.size,
                CameraMode::O,
                self.camera.fov,
            ),
            _=>unimplemented!()
        }
    }
    pub fn create_intergator(&self) -> Integrator {
        match self.intergator {
            IntegratorToml::Direct {
                core_num,
                sample_num,
                startegy,
            } => {
                let direct = Box::new(DirectIntegrator::new(0, startegy, Sampler::new(1)));
                Integrator::Direct(direct, core_num, Sampler::new(sample_num))
            }
            IntegratorToml::Path {
                core_num,
                sample_num,
                q,
                max_depth,
            } => Integrator::Path(
                Box::new(PathIntegrator::new(q, max_depth)),
                core_num,
                Sampler::new(sample_num),
            ),
        }
    }
    pub fn create_setting(&self) -> Setting {
        match self.intergator {
            IntegratorToml::Direct {
                core_num,
                sample_num,
                startegy,
            } => Setting::new(
                core_num,
                self.name.to_owned(),
                self.camera.size.as_uvec2(),
                "direct".to_ascii_lowercase(),
            ),
            IntegratorToml::Path {
                core_num,
                sample_num,
                q,
                max_depth,
            } => Setting::new(
                core_num,
                self.name.to_owned(),
                self.camera.size.as_uvec2(),
                "path".to_ascii_lowercase(),
            ),
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
#[derive(Deserialize, Debug, Serialize, Default)]

pub struct TransformToml {
    r: Vec4,
    s: Vec3,
    t: Vec3,
}
impl TransformToml {
    pub fn get_mat(&self) -> Mat4 {
        let angle = self.r.w.to_radians();
        let quat = Quat::from_axis_angle(self.r.truncate(), angle);
        Mat4::from_scale_rotation_translation(self.s, quat, self.t)
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
    Fourier {},
}
#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum TextureToml {
    Image { path: String },
    Constant { value: Vec3 },
    // Value{value:f32}
}
#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum LightToml {
    Point {
        trans: TransformToml,
        point: Vec3,
        lemit: Vec3,
    },
    Spot {
        trans: TransformToml,
        point: Vec3,
        lemit: Vec3,
        end_angle: f32,
        start_angle: f32,
    },
    Texture {
        lemit: Vec3,
        texture: TextureToml,
    },
    Distant {
        lemit: Vec3,
        dir: Vec3,
        world_center: Vec3,
        world_radius: Vec3,
    },
    Area {
        trans: TransformToml,
        lemit: Vec3,
        shape_index: usize,
    },
    Infinite {
        skybox: TextureToml,
        lemit: Vec3,
        world_center: Vec3,
        world_radius: Vec3,
    },
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
