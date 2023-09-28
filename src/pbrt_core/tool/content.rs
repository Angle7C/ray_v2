use std::{
    fs::File,
    io::{BufReader, Read},
    ops::Deref,
};

use crossbeam::epoch::Pointable;
use glam::{Vec2, UVec2};

use crate::pbrt_core::{integrator::{SampleIntegrator, self}, load::TomlLoad};

use super::{sence::{self, Sence}, film::Film};

#[derive(Default)]
pub struct Setting {
    pub name: String,
    pub num: u32,
    pub core_num: u32,
    pub size:UVec2
}
#[derive(Default)]
pub struct Content {
    sence: Sence,
    setting: Setting,
    integrator: Option<SampleIntegrator>,
}
impl Content{
    pub fn new(path: &str) -> Content {
        let mut file = File::open(path).expect("读取文件失败");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("读取文件失败");

        let load = toml::from_str::<TomlLoad>(&buf).unwrap();
        let mut sence = Sence::default();
        let setting = load.crate_setting();
        let integrator = load.crate_integrator();
        load.build(&mut sence);
        Content {
            sence,
            setting,
            integrator: Some(integrator),
        }
    }
    pub fn render(self) {
        if let Some(integrator) = self.integrator {
            let film = Film::new(self.setting.size);
            
            integrator.render(self.sence, film, self.setting);
        }
    }
}
