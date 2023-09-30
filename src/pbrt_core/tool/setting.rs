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