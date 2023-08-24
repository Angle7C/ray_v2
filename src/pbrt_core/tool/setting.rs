use std::fs::File;

use glam::{UVec2, DVec3, DVec2, DMat4};
use serde_json::Value;

use crate::pbrt_core::{camera::{Camera, CameraMode}, load::GltfLoad, primitive::{shape::Shape, bvh::BVH}, light::{Light, point::PointLight}};

use super::sence::Sence;

pub struct Setting{
    core_num: u64,
    name: String,
    size:UVec2,
    sample_num:u64,
    path:String
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
        Self { core_num, name, size, sample_num, path }
    }
}
impl Parse for Camera{
    fn parse(value:&Value)->Self {
        let eye = DVec3::parse(&value["eye"]);
        let center=DVec3::parse(&value["center"]);
        let up=DVec3::parse(&value["up"]);
        let size=DVec2::parse(&value["size"]);
        let mode=if value["mode"].as_str().unwrap().contains("O"){
            CameraMode::O
        }else{
            CameraMode::P
        };
        let fov=f64::parse(&value["fov"]);
        Camera::new(eye, center, up, size.as_vec2(), mode, fov)
    }
}
pub struct Build;
impl Build{
    pub fn build(path:&str)->Sence{
        let buf = File::open(path).unwrap();
        let json:Value = serde_json::from_reader(buf).unwrap();
        let setting = Setting::parse(&json["setting"]);
        let camera=Camera::parse(&json["camera"]);
        let shape=GltfLoad::load(&setting.path);
        let mut light=vec![];
        light.push(Light::PointLight(Box::new(PointLight::new(DVec3::splat(100.0),DVec3::splat(5.0),DMat4::IDENTITY))));
        Sence::new(shape, light, camera)

    }
}
impl Parse for Vec<Shape>{
    fn parse(value:&Value)->Self {
        unimplemented!()
    }
}