use log::info;
use pbrt_core::tool::{setting::Build, log::log_init};

#[allow(unused, dead_code)]
pub mod pbrt_core;

mod test;

pub fn main() {
    log_init();
    info!("******************************");
    let build= Build::build("./file/sence.json");
    #[cfg(not(debug_assertions))]
        build.render();
    #[cfg(debug_assertions)]
        build.render_debug();
    info!("-------------------------------");

}
