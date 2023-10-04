
use pbrt_core::tool::{log::log_init, build::Context};

pub mod pbrt_core;

mod test;

pub fn main() {
    log_init();

    let context= Context::new("./file/box.toml");
    context.render();

}
