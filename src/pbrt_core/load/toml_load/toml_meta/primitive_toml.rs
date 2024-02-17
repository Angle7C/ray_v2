
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::pbrt_core::light::LightAble;
use crate::pbrt_core::material::Material;
use crate::pbrt_core::primitive::shape::ShapeAble;
use crate::pbrt_core::primitive::GeometricPrimitive;

#[derive(Deserialize,Serialize,Debug)]
#[serde(tag = "mode")]
pub enum PrimitiveToml{
    //球
    GeometricPrimitive{
        shape_index:usize,
        material_index:Option<usize>,
        light_index:Option<usize>
    },
}
impl PrimitiveToml{
    pub fn get(&self, materials:&[Arc<dyn Material>],shapes:&[Arc<dyn ShapeAble>],lights:&[Arc<dyn LightAble>]) ->GeometricPrimitive{
       match *self {
        PrimitiveToml::GeometricPrimitive { shape_index, material_index, light_index } =>{
            let shape=shapes.get(shape_index).cloned().expect("获取shape失败");
            let material=material_index.and_then(|index|
                    materials.get(index).cloned());
            let light=light_index.and_then(|index|lights.get(index).cloned());
     
            GeometricPrimitive::new(shape, material, light, None)
        },
    }
    }
}
