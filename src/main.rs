
use pbrt_core::{tool::{log::log_init, build::Context}, load::Load};

pub mod pbrt_core;

mod test;

pub fn main() {
    log_init();

    let context= Load::load("./file/setting.toml").unwrap();
    context.render();

}