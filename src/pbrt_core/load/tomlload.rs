use std::{path::Path, sync::Arc};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    material::{
        self, matte::Matte, metal::MetalMaterial, mirror::Mirror, plastic::Plastic, Material,
    },
    primitive::{self, Primitive},
    texture::{constant::ConstantTexture, image::ImageTexture, Texture},
    tool::mipmap::{ImageData, MipMap},
};

use super::{
    myload::{LightToml, MaterialToml, ShapeToml, TextureToml, TransformToml},
    objload::ObjLoad,
};
#[derive(Deserialize, Debug, Serialize, Default)]
struct ObjToml {
    pub transform: TransformToml,
    pub objtype: String,
    pub path: String,
    pub material_index: usize,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct TomlLoader {
    object: Vec<ObjToml>,
    material: Vec<MaterialToml>,
    texture: Vec<TextureToml>,
    light: Vec<LightToml>,
    shape: Vec<ShapeToml>,
}
impl TomlLoader {
    pub fn load_sence<'b, 'a>(self)
    where
        'b: 'a,
    {
        let textures = Self::load_texture(self.texture).unwrap();
        let materials = Self::load_material(self.material, textures).unwrap();
    }
    fn load_texture<'a>(textures: Vec<TextureToml>) -> Result<Vec<Arc<dyn Texture + 'a>>> {
        let mut vec = vec![];
        for texture in textures {
            let box_texture: Arc<dyn Texture> = match texture {
                TextureToml::Image { path } => {
                    let path = Path::new("image").join(path);
                    let image = image::io::Reader::open(path)?.decode()?;
                    let image_data = ImageData::new_dynimage(image);
                    let mipmap = MipMap::new(image_data);
                    Arc::new(ImageTexture::new(mipmap))
                }
                TextureToml::Constant { value } => Arc::new(ConstantTexture::new(value)),
            };
            vec.push(box_texture);
        }
        Ok(vec)
    }
    fn load_material<'a>(
        materials: Vec<MaterialToml>,
        texture: Vec<Arc<dyn Texture + 'a>>,
    ) -> Result<Vec<Box<dyn material::Material + 'a>>> {
        let mut vec = vec![];
        for material in materials {
            let a: Box<dyn Material> = match material {
                MaterialToml::Matte { kd, sigma } => {
                    let kd = texture.get(kd).unwrap().clone();
                    // let sigma = texture.get(*sigma);
                    Box::new(Matte::new(kd, sigma))
                }
                MaterialToml::Plastic { kd, ks, roughness } => {
                    let kd = texture.get(kd).unwrap();
                    let ks = texture.get(ks).unwrap();
                    let roughness = texture.get(roughness).unwrap();
                    Box::new(Plastic::new(kd.clone(), ks.clone(), roughness.clone()))
                }
                MaterialToml::Mirror { kr } => {
                    let kr = texture.get(kr).unwrap();
                    Box::new(Mirror::new(kr.clone()))
                }
                MaterialToml::Metal { eta, k, roughness } => {
                    let eta = texture.get(eta).unwrap();
                    let k = texture.get(k).unwrap();
                    let roughness = texture.get(roughness).unwrap();
                    Box::new(MetalMaterial::new(
                        eta.clone(),
                        k.clone(),
                        roughness.clone(),
                        false,
                    ))
                }
                _ => todo!(),
            };
            vec.push(a)
        }
        Ok(vec)
    }
    fn load_object<'a>(
        objects: Vec<ObjToml>,
        materials: Vec<Box<dyn material::Material + 'a>>,
    ) -> Result<Vec<Box<dyn Primitive + 'a>>> {
        let mut vec = vec![];
        // for object in objects {

        //     vec.push(a);
        // };
        Ok(vec)
    }
    fn load_sigle_object<'a>(
        object: ObjToml,
        materials: Vec<Box<dyn material::Material + 'a>>,
    ) -> Result<Box<dyn Primitive + 'a>> {
        let trans = object.transform.get_mat();
        let material = materials.get(object.material_index).unwrap();
        let object_path = object.path;
        let objype = object.objtype;
        // let a=match objype.as_str(){
        //     // "obj"=>ObjLoad::load(&object_path,material,trans),
        //     // _=>todo!(),
        //     unimplemented!();
        // };
        // Ok(a)
        unimplemented!()
    }
}
