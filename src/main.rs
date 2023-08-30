use pbrt_core::tool::setting::Build;

#[allow(unused, dead_code)]
pub mod pbrt_core;

mod test;

pub fn main() {
    let build= Build::build("./sence/sence.json");
    #[cfg(not(debug_assertions))]
        build.render();
    #[cfg(debug_assertions)]
        build.render_debug()
}
