use glam::UVec2;
use pbrt_core::{tool::setting::Build, integrator::path::PathIntegrator};

#[allow(unused,dead_code)]
pub mod pbrt_core;

mod test;

pub fn main(){
    let sence = Build::build("./sence/sence.json");
    let path = PathIntegrator::default();
    #[cfg(not(debug_assertions))]
    path.render_process("test", 6, UVec2::new(512, 512), &sence);
    #[cfg(debug_assertions)]
    path.render_process_debug("test", 6, UVec2::new(512, 512), &sence);

}