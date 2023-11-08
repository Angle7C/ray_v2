use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::pbrt_core::camera::Camera;

use self::{
    myload::{CameraToml, IntegratorToml},
    tomlload::TomlLoader,
};

use super::{
    camera::CameraMode,
    integrator::{direct::DirectIntegrator, path::PathIntegrator, Integrator},
    sampler::Sampler,
    tool::sence::Sence,
};

pub mod gltfload;
pub mod myload;
pub mod objload;
pub mod tomlload;
#[derive(Deserialize, Debug, Serialize)]
struct LoadData {
    pub path: String,
    pub name: String,
    pub camera: CameraToml,
    pub intergator: IntegratorToml,
}

pub struct Load;
impl Load {
    pub fn load(path: &str) -> anyhow::Result<Sence> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let data: LoadData = toml::from_str(&buf)?;
        Self::load_camera(&data.camera);
        Self::create_intergator(&data.intergator);
        Self::build_sence(&data.path)
    }
    fn build_sence(path: &str) -> anyhow::Result<Sence> {
        match path.split(".").last().unwrap() {
            "toml" => {
                Self::toml_load_sence(path);
            }
            _ => unimplemented!("文件toml类型暂不支持"),
        }
        unimplemented!();
        // Err("读取Sence失败");
    }
    fn toml_load_sence(path: &str) -> anyhow::Result<Sence> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let loader: TomlLoader = toml::from_str(&buf)?;
        loader.load_sence()
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
}
