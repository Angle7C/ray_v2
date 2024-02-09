use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::pbrt_core::material::Material;
use crate::pbrt_core::material::matte::Matte;
use crate::pbrt_core::material::metal::MetalMaterial;
use crate::pbrt_core::material::mirror::Mirror;
use crate::pbrt_core::material::plastic::Plastic;
use crate::pbrt_core::texture::Texture;


#[derive(Deserialize, Debug, Serialize,Clone,Copy)]
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
    Metal {
        eta: usize,
        k: usize,
        roughness: usize,
    },
}
impl MaterialToml{
    pub fn get(&self,textures:&[Arc<dyn Texture>])->Arc<dyn Material>{

       let material:Arc<dyn Material>= match *self{
            //漫反射材质
            Self::Matte { kd,sigma } => {
                let kd = textures.get(kd).expect("获取纹理失败");
                Arc::new( Matte::new(kd.clone(),sigma))
            }
            //塑料材质
            Self::Plastic { kd,ks,roughness } => {
                let kd = textures.get(kd).expect("获取纹理失败");
                let ks = textures.get(ks).expect("获取纹理失败");
                let roughness = textures.get(roughness).expect("获取纹理失败");
                Arc::new(Plastic::new(kd.clone(),ks.clone(),roughness.clone()))

            }
            //镜面材质
            Self::Mirror { kr } => {
                let kr = textures.get(kr).expect("获取纹理失败");
                Arc::new(Mirror::new(kr.clone()))
            }
            //金属材质
            Self::Metal { eta,k,roughness } => {
                let eta = textures.get(eta).expect("获取纹理失败");
                let k = textures.get(k).expect("获取纹理失败");
                let roughness = textures.get(roughness).expect("获取纹理失败");
                Arc::new(MetalMaterial::new(eta.clone(),k.clone(),roughness.clone(),false))
            }
        };
        material
    }
}