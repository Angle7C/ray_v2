use glam::UVec2;



use super::sence::Sence;

pub struct Setting {
    pub core_num: u64,
    pub name: String,
    pub size: UVec2,
    pub sample_num: u64,
    pub path: String,
    pub inter_mode: String,
}
impl Setting {
    pub fn new(core: usize, name: String, size: UVec2, inter_mode: String) -> Self {
        Self {
            core_num: core as u64,
            name,
            size,
            sample_num: 1,
            path: "".to_owned(),
            inter_mode,
        }
    }
}
pub struct Build {
    sence: Sence,
    setting: Setting,
}
// impl Build {
//     pub fn render(self) {
//         let integrator = if self.setting.inter_mode.contains("path") {
//             Integrator::Path(Box::new(PathIntegrator::new(
//                 0.8,
//                 5,
//                 Sampler::new(self.setting.sample_num as usize),
//                 self.setting.size,
//             )))
//         } else {
//             Integrator::Direct(Box::new(DirectIntegrator::new(
//                 1,
//                 LightStartegy::UniformOne,
//                 Sampler::new(self.setting.sample_num as usize),
//             )))
//         };

//         integrator.render_process(
//             &self.setting.name,
//             self.setting.core_num,
//             &self.sence,
//             self.setting.size,
//             Sampler::new(self.setting.sample_num as usize),
//         )
//     }
//     pub fn render_debug(self) {
//         let integrator = if self.setting.inter_mode.contains("path") {
//             Integrator::Path(Box::new(PathIntegrator::new(
//                 0.8,
//                 5,
//                 Sampler::new(self.setting.sample_num as usize),
//                 self.setting.size,
//             )))
//         } else {
//             Integrator::Direct(Box::new(DirectIntegrator::new(
//                 1,
//                 LightStartegy::UniformOne,
//                 Sampler::new(self.setting.sample_num as usize),
//             )))
//         };
//         integrator.render_process_debug(
//             &self.setting.name,
//             self.setting.core_num,
//             &self.sence,
//             self.setting.size,
//         );
//     }
// }
