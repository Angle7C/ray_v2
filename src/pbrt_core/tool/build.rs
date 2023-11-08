use std::{fs::File, io::Read};

use crate::pbrt_core::{integrator::Integrator, load::myload::MyLoad};

use super::{sence::Sence, setting::Setting};

pub struct Context {
    sence: Sence,
    intergator: Integrator,
    setting: Setting,
}
impl Context {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf);
        let load = toml::from_str::<MyLoad>(&buf).unwrap();
        let intergator = load.create_intergator();
        let sence = load.load_sence();
        let setting = load.create_setting();
        Self {
            sence,
            intergator,
            setting,
        }
    }
    pub fn render(self) {
        self.intergator
            .render_process(&self.setting.name, &self.sence, self.setting.size)
    }
}
