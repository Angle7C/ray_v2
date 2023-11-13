use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::pbrt_core::{camera::Camera, load::objload::ObjLoad};

use self::{
    myload::{CameraToml, IntegratorToml},
    tomlload::TomlLoader,
};

use super::{
    camera::CameraMode,
    integrator::{direct::DirectIntegrator, path::PathIntegrator, Integrator},
    sampler::Sampler,
    tool::{
        build::Context,
        sence::Sence,
        setting::{self, Setting},
    },
};

pub mod gltfload;
pub mod myload;
pub mod objload;

pub mod jsonload;
pub mod tomlload;
#[derive(Deserialize, Debug, Serialize,Default)]
struct LoadData {
    pub path: String,
    pub name: String,
    pub camera: CameraToml,
    pub intergator: IntegratorToml,
}

pub struct Load;
impl Load {
    pub fn load(path: &str) -> anyhow::Result<Context> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let data: LoadData = toml::from_str(&buf)?;
        let camera = Self::load_camera(&data.camera);
        let integrator = Self::create_intergator(&data.intergator);
        let sence = Self::build_sence(&data.path, camera)?;
        let setting = Self::create_setting(&data.intergator, data.name, &data.camera);
        Ok(Context::new(sence, integrator, setting))
    }
    fn build_sence(path: &str, camera: Camera) -> anyhow::Result<Sence> {
        let sence = match path.split(".").last().unwrap() {
            "toml" => Self::toml_load_sence(path, camera),
            "json" => {
                unimplemented!()
            }
            _ => unimplemented!("文件类型暂不支持"),
        };
        sence
    }
    fn toml_load_sence(path: &str, camera: Camera) -> anyhow::Result<Sence> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let loader: TomlLoader = toml::from_str(&buf)?;
        Ok(loader.load_sence(camera))
    }
    fn load_camera(camera: &CameraToml) -> Camera {
        let mode = camera.mode.as_str();
        match mode {
            "P" => Camera::new(
                camera.eye,
                camera.target,
                camera.up,
                camera.size,
                CameraMode::P,
                camera.fov,
            ),
            "O" => Camera::new(
                camera.eye,
                camera.target,
                camera.up,
                camera.size,
                CameraMode::O,
                camera.fov,
            ),
            _ => unimplemented!("不支持其他类型Camera"),
        }
    }
    fn create_intergator(integrator: &IntegratorToml) -> Integrator {
        match *integrator {
            IntegratorToml::Direct {
                core_num,
                sample_num,
                startegy,
            } => {
                let direct = Box::new(DirectIntegrator::new(0, startegy, Sampler::new(1)));
                Integrator::Direct(direct, core_num, Sampler::new(sample_num))
            }
            IntegratorToml::Path {
                core_num,
                sample_num,
                q,
                max_depth,
            } => Integrator::Path(
                Box::new(PathIntegrator::new(q, max_depth)),
                core_num,
                Sampler::new(sample_num),
            ),
        }
    }
    pub fn create_setting(
        integrator: &IntegratorToml,
        name: String,
        camera: &CameraToml,
    ) -> Setting {
        match integrator {
            IntegratorToml::Direct {
                core_num,
                sample_num: _,
                startegy: _,
            } => Setting::new(
                *core_num,
                name.to_owned(),
                camera.size.as_uvec2(),
                "direct".to_ascii_lowercase(),
            ),
            IntegratorToml::Path {
                core_num,
                sample_num: _,
                q: _,
                max_depth: _,
            } => Setting::new(
                *core_num,
                name.to_owned(),
                camera.size.as_uvec2(),
                "path".to_ascii_lowercase(),
            ),
        }
    }
}
#[cfg(test)]
mod test{
    use crate::pbrt_core::camera::Camera;

    use super::LoadData;
    #[test]
    pub fn test_data(){
        let mut data = LoadData::default();
        data.name=String::from("value");
        let str = toml::to_string(&data).unwrap();
        println!("{:?}",data)
    }
}