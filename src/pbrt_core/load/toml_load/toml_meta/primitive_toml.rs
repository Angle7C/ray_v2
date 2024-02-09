
use std::default;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::pbrt_core::load::toml_load::toml_meta::base_toml::TransformToml;
use crate::pbrt_core::material::Material;
use crate::pbrt_core::primitive::shape::rectangle::Rectangle;
use crate::pbrt_core::primitive::Primitive;
use crate::pbrt_core::primitive::shape::sphere::Sphere;

#[derive(Deserialize,Serialize,Debug)]
#[serde(tag = "mode")]
pub enum PrimitiveToml{
    //球
    Sphere{
        transform:TransformToml,
        r:f32,
        material_index:usize
    },
    //矩形
    Rect{
        transform:TransformToml,
        material_index:usize

    },
    //圆环
    Disk{
        transform:TransformToml,
        inner_r:f32,
        outer_r:f32,
        material_index:usize
    }
}
impl PrimitiveToml{
    pub fn get(&self, materials:&[Arc<dyn Material>]) ->Arc<dyn Primitive>{
       match *self {
            PrimitiveToml::Sphere {
                transform,
                r,
                material_index
            } => {
                let material = materials.get(material_index).expect("获取材质失败").clone();
                let mat = transform.get();
                Arc::new( Sphere::new(r, Some(material), mat))
            }
            PrimitiveToml::Rect { transform,material_index } => {
                let material = materials.get(material_index).expect("获取材质失败").clone();
                let mat = transform.get();
                Arc::new(Rectangle::new(mat, Some(material)))
                
            }
            PrimitiveToml::Disk { .. } => {todo!()}
        }
    }
}