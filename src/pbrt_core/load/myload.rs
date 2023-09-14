use std:: pin::Pin;

use glam::{Vec3,Vec2,Vec4};
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    texture::{constant::ConstantTexture, image::ImageTexture, Texture},
    tool::
        mipmap::{ImageData, MipMap},
};
// use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug, Serialize)]
pub struct MyLoad {
    camera: CameraToml,
    shapes: Vec<ShapeToml>,
    material: Vec<MaterialToml>,
    light: Vec<LightToml>,
    texture: Vec<TextureToml>,
}
impl MyLoad {
    pub fn load_sence(self) {}
    fn load_material(&mut self) {
        unimplemented!()
    }
    fn load_texture(&self) -> Pin<Box<Vec<Box<dyn Texture>>>> {
        let mut vec=vec![];
        for texture in &self.texture {
            let box_texture: Box<dyn Texture> = match texture {
                TextureToml::Image { path } => {
                    let image = image::io::Reader::open(path).expect("").decode().expect("");
                    let image_data = ImageData::new_dynimage(image);
                    let mipmap = MipMap::new(image_data);
                    Box::new(ImageTexture::new(mipmap))
                }
                TextureToml::Constant { value } => Box::new(ConstantTexture::new(*value)),
            };
            vec.push(box_texture);
        }
        Box::pin(vec)
    }
    fn load_shape(&mut self) {}
    fn load_light(&mut self) {}
    fn load_env(&mut self) {}
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CameraToml {
    mode: String,
    size: Vec2,
    far: f32,
    near: f32,
}
#[derive(Deserialize, Debug, Serialize)]

pub struct TransformToml {
    r: Vec4,
    s: Vec3,
    t: Vec3,
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
        kd: TextureToml,
        sigma_angle: TextureToml,
    },
    Plastic {
        kd: TextureToml,
        ks: TextureToml,
        roughness: TextureToml,
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
