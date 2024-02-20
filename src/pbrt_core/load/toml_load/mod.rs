use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::pbrt_core::camera::Camera;
use crate::pbrt_core::light::LightAble;
use crate::pbrt_core::load::LoadSceneAble;
use crate::pbrt_core::load::toml_load::toml_meta::base_toml::{CameraToml, IntegratorToml};
use crate::pbrt_core::load::toml_load::toml_meta::light_toml::LightToml;
use crate::pbrt_core::load::toml_load::toml_meta::material_toml::MaterialToml;
use crate::pbrt_core::load::toml_load::toml_meta::primitive_toml::PrimitiveToml;
use crate::pbrt_core::load::toml_load::toml_meta::texture_toml::TextureToml;
use crate::pbrt_core::material::Material;
use crate::pbrt_core::primitive::shape::ShapeAble;
use crate::pbrt_core::primitive::{GeometricPrimitive};
use crate::pbrt_core::texture::Texture;
use crate::pbrt_core::tool::build::Context;
use crate::pbrt_core::tool::sence::Scene;

use self::toml_meta::shape_toml::ShapeToml;

pub mod toml_meta;

pub struct TomlLoader;

#[derive(Deserialize,Serialize,Default,Debug)]
struct MetalData{
    //图片名称
    name:String,
    //模型资源
    assert:String,
    //相机设置
    camera:CameraToml,
    //渲染器设置
    integrator:IntegratorToml,
}
#[derive(Deserialize,Serialize,Default,Debug)]
struct  TomlMetalData{
    //纹理设置
    texture:Vec<TextureToml>,
    //材质设置
    material:Vec<MaterialToml>,
    //光源设置
    light:Vec<LightToml>,
    //几何设置
    shape:Vec<ShapeToml>,
    //图元设置
    primitive:Vec<PrimitiveToml>
}


impl LoadSceneAble for TomlLoader{
    fn load(&self, data: &[u8]) -> anyhow::Result<Context> {
        //获取元数据
        let data = String::from_utf8_lossy(data).to_string();
        let metal = toml::from_str::<MetalData>(&data)?;
        let integrator = metal.integrator.get();
        let camera = metal.camera.get();
        let scene = self.create_scene(&metal, camera);

        //构建上下文
        let context=Context::new(integrator,metal.name,metal.camera.size.as_uvec2(),scene);
        Ok(context)

    }
}
impl TomlLoader{
    fn create_scene(&self,meta: &MetalData,camera:Camera)->Scene{
        //加载资源数据
        let mut file = File::open(Path::new(&meta.assert)).expect("资源路径打开失败");
        let mut buf=String::new();
        file.read_to_string(&mut buf).expect("读取scene资源失败");
        let toml_metal = toml::from_str::<TomlMetalData>(&buf).expect("scene文件内容不合法");

        let textures = self.load_texture(toml_metal.texture);
        let materials=self.load_material(toml_metal.material,&textures);
        let shapes=self.load_shape(toml_metal.shape);
        let lights=self.load_light(toml_metal.light, &textures);
        let primitive = self.build_primitive(toml_metal.primitive, &materials, &lights, &shapes);
        Scene::new(primitive,camera)


    }
    //加载纹理图片
    fn load_texture(&self,textures:Vec<TextureToml>)->Vec<Arc<dyn Texture>>{

        textures.into_iter()
            .map(|item| item.get())
            .collect::<Vec<_>>()
    }
    fn load_material(&self,materials: Vec<MaterialToml>,textures:&[Arc<dyn Texture>])->Vec<Arc<dyn Material>>{
        materials.into_iter()
            .map(|item|item.get(textures))
            .collect::<Vec<_>>()
    }
    fn load_light(&self,lights:Vec<LightToml>,textures:&[Arc<dyn Texture>])->Vec<Arc<dyn LightAble>>{
        lights.into_iter()
            .map(|item|item.get(textures))
            .collect::<Vec<_>>()
    }
    fn load_shape(&self,shapes: Vec<ShapeToml>,)->Vec<Arc<dyn ShapeAble>>{
        shapes.into_iter()
                .map(|item|item.get())
                .collect::<Vec<_>>()
    }
    fn build_primitive(&self,primitives: Vec<PrimitiveToml>,materials:&[Arc<dyn Material>],lights:&[Arc<dyn LightAble>], shapes: &[Arc<dyn ShapeAble>])->Vec<GeometricPrimitive>{
        primitives.iter()
        .map(|item|item.get(materials, shapes, lights))
        .collect::<Vec<_>>()
    }

}