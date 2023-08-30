use std::fs::File;

use glam::{UVec2, DVec3, DVec2, DMat4, Vec2, DVec4, DQuat};
use serde::de::value;
use serde_json::Value;

use crate::pbrt_core::{camera::{Camera, CameraMode, self}, load::GltfLoad, primitive::{shape::{Shape, rectangle::Rectangle}, bvh::BVH, Primitive}, light::{Light, area::DiffuseAreaLight, self}, integrator::{path::PathIntegrator, Integrator}, sampler::Sampler};

use super::sence::Sence;

pub struct Setting{
    pub core_num: u64,
    pub name: String,
    pub size:UVec2,
    pub sample_num:u64,
    pub path:String,
    pub inter_mode:String
}
pub trait Parse {
    fn parse(value:&Value)->Self;
}
impl Parse for DVec3{
    fn parse(value: &Value) -> Self {
        let x = value["x"].as_f64().unwrap_or_default() as f64;
        let y = value["y"].as_f64().unwrap_or_default() as f64;
        let z = value["z"].as_f64().unwrap_or_default() as f64;
        DVec3::new(x,y,z)
    }
}
impl Parse for DVec2{
    fn parse(value: &Value) -> Self {
        let x = value["x"].as_f64().unwrap_or_default() as f64;
        let y = value["y"].as_f64().unwrap_or_default() as f64;
        DVec2::new(x,y)
    }
}
impl Parse for UVec2{
    fn parse(value: &Value) -> Self {
        let x = value["x"].as_u64().unwrap_or_default() as u32;
        let y = value["y"].as_u64().unwrap_or_default() as u32;
        UVec2 { x, y }
    }
}
impl Parse for f64{
    fn parse(value: &Value) -> Self {
        value.as_f64().unwrap_or_default() as f64
    }
}
impl Parse for u64{
    fn parse(value:&Value)->Self {
        value.as_u64().unwrap_or_default()
    }
}
impl Parse for Setting{
    fn parse(value:&Value)->Self {
        let core_num=u64::parse(&value["core_num"]);           
        let size=UVec2::parse(&value["size"]);
        let name=value["name"].as_str().unwrap().to_owned();
        let path=value["path"].as_str().unwrap().to_owned();
        let sample_num=u64::parse(&value["sample_num"]);     
        let inter_mode=value["inter_mode"].as_str().unwrap().to_owned();      
        Self { core_num, name, size, sample_num,path, inter_mode }
    }
}
impl Parse for Camera{
    fn parse(value:&Value)->Self {
        let eye = DVec3::parse(&value["eye"]);
        let center=DVec3::parse(&value["center"]);
        let up=DVec3::parse(&value["up"]);
        let mode=if value["mode"].as_str().unwrap().contains("O"){
            CameraMode::O
        }else{
            CameraMode::P
        };
        let fov=f64::parse(&value["fov"]);
        Camera::new(eye, center, up, Vec2::new(512.0, 512.0), mode, fov)
    }
}
impl Parse for DVec4{
    fn parse(value:&Value)->Self {
        let x = value["x"].as_f64().unwrap_or_default() as f64;
        let y = value["y"].as_f64().unwrap_or_default() as f64;
        let z = value["z"].as_f64().unwrap_or_default() as f64;
        let w = value["w"].as_f64().unwrap_or_default() as f64;
        DVec4 { x, y, z, w }
    }
}
impl Parse for DQuat{
     fn parse(value:&Value)->Self {
        let x = value["x"].as_f64().unwrap_or_default() as f64;
        let y = value["y"].as_f64().unwrap_or_default() as f64;
        let z = value["z"].as_f64().unwrap_or_default() as f64;
        let w = value["w"].as_f64().unwrap_or_default() as f64;
        DQuat { x, y, z, w }
    }
}
pub struct Build<'a>{
    sence:Sence<'a>,
    setting:Setting,
}
impl<'a> Build<'a>{
    pub fn render(self){
        let path = Integrator::Path(Box::new(PathIntegrator::new(0.8, 5, Sampler::default(), self.setting.size)));
        path.render_process(&self.setting.name, self.setting.core_num, &self.sence, self.setting.size)
    }
    pub fn render_debug(self){
        let path = Integrator::Path(Box::new(PathIntegrator::new(0.8, 5, Sampler::default(), self.setting.size)));
        path.render_process_debug(&self.setting.name, self.setting.core_num, &self.sence, self.setting.size,Default::default());
    }
    pub fn build(path:&str)->Self{
        let buf = File::open(path).unwrap();
        let json:Value = serde_json::from_reader(buf).unwrap();
        let setting = Setting::parse(&json["setting"]);
        let light=Self::get_light(&json["lights"]);
        let primitive=Self::get_primitive(&setting.path);
        let camera=Self::get_camera(&json["camera"], setting.size);
        let sence = Sence::new(primitive, light, camera);
        Self { sence, setting }
    }
    fn get_light(value:&Value)->Vec<Light>{
        let mut light_vec=vec![];
        for light in value.as_array().unwrap() {
            if !value.is_null(){
                light_vec.push(Light::parse(light))
            }
        };
        light_vec
    }
    fn get_camera(value:&Value,size:UVec2)->Camera{
        let mut camera=Camera::parse(value);
        camera.reset_size(size.as_vec2());
        camera
    }
    fn get_primitive(path:&str)->Vec<Box<dyn Primitive>>{
        GltfLoad::load(path)
    }
}
impl Parse for Light{
    fn parse(value:&Value)->Self {
    
        let mode = value["mode"].as_str().unwrap_or_else(||"");
        let light=if mode.contains("diffuse"){
            let shape=Shape::parse(&value["shape"]);
            let lemit=DVec3::parse(&value["lemit"]);
            let diffuse = DiffuseAreaLight::new(lemit, shape);
            Light::AreaLight(Box::new(diffuse))
        }else{
            unimplemented!("不支持该类型光源")
        };
        light
    }
}

impl Parse for Shape{
    fn parse(value:&Value)->Self {
        let mode = value["mode"].as_str().unwrap_or_else(||"");
        let shape=if mode.contains("rect"){
            Shape::Rect( Rectangle::parse(value))
        }else{
            unimplemented!("不支持该类型shape")
        };
        return shape
    }
}