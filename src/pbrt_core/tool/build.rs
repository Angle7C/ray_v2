use std::{fs::File, io::Read};

use crate::pbrt_core::{integrator::Integrator, load::Load};

use super::{sence::Sence, setting::Setting};

pub struct Context {
    sence: Sence,
    intergator: Integrator,
    setting: Setting,
}
impl Context {
    pub fn render(self) {
        self.intergator
            .render_process(&self.setting.name, &self.sence, self.setting.size)
    }
    pub fn new(sence: Sence, intergator: Integrator, setting: Setting) -> Self {
        Self {
            sence,
            intergator,
            setting,
        }
    }
}
