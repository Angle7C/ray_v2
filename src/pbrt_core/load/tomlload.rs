use std::{ops::Add, path::Path, sync::Arc};

use anyhow::Result;
use glam::UVec3;
use rand::seq::index;
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    material::{
        self, matte::Matte, metal::MetalMaterial, mirror::Mirror, plastic::Plastic, Material,
    },
    primitive::{self, mesh::Mesh, shape::triangle::Triangle, Primitive},
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
        let primitive=Self::load_object(self.object, materials).unwrap();
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
        let mut all_mesh = Default::default();
        //先获取对应obj的mesh，index。并存储最后进行合并计算
        for object in objects {
            let obj_to_world = object.transform.get_mat();
            let material = materials.get(object.material_index).unwrap();
            let vec = Self::load_sigle_object(object, &mut all_mesh)?;
        }
        Ok(vec)
    }
    fn load_sigle_object<'a>(object: ObjToml, all_mesh: &mut Mesh) -> Result<Vec<Vec<UVec3>>> {
        let object_path = object.path;
        let objtype = object.objtype;
        let (mut mesh, vec) = match objtype.as_str() {
            "obj" => ObjLoad::load(&object_path),
            _ => unimplemented!(),
        }?;
        let mut ans_index = vec![];
        for (index, item) in vec.iter().enumerate() {
            let size = get_type_index(index, &all_mesh);
            let item = item
                .iter()
                .map(|v| v.add(UVec3::splat(size as u32)))
                .collect::<Vec<_>>();
            ans_index.push(item);
        }
        all_mesh.merge(&mut mesh);
        Ok(ans_index)
    }
}
#[inline]
fn get_type_index(index: usize, mesh: &Mesh) -> usize {
    match index {
        0 => mesh.pos_size(),
        1 => mesh.norm_size(),
        2 => mesh.uv_size(),
        _ => panic!(),
    }
}
