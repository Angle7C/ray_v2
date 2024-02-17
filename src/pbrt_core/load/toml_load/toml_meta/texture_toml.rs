use std::path::Path;
use std::sync::Arc;
use glam::Vec3;
use image::io::Reader as ImageReader;
use serde::{Deserialize, Serialize};
use crate::pbrt_core::texture::constant::ConstantTexture;
use crate::pbrt_core::texture::image::ImageTexture;
use crate::pbrt_core::texture::Texture;
use crate::pbrt_core::tool::color::RGB;
use crate::pbrt_core::tool::mipmap::{ImageData, MipMap};


#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum TextureToml {
    Image { path: String },
    Constant { value: RGB },
}
impl TextureToml{
    pub fn get(self)->Arc<dyn Texture>{

       match self {
            //图片纹理
           TextureToml::Image{ path} => {
                let image = ImageReader::open(Path::new(&path))
                    .expect("读取纹理图片失败")
                    .decode()
                    .expect("解码纹理图片失败");
                let mipmap = MipMap::new(ImageData::new_dynimage(image));
               Arc::new(ImageTexture::new(mipmap))
            }
            //常量纹理
            TextureToml::Constant{value } =>
                Arc::new(ConstantTexture::new(value))
        }
    }
}