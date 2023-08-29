use pbrt_core::tool::setting::Build;

#[allow(unused, dead_code)]
pub mod pbrt_core;

mod test;

pub fn main() {
    let (sence, path, setting) = Build::build("./sence/sence.json");
    #[cfg(not(debug_assertions))]
    path.render_process(
        &setting.name,
        setting.sample_num,
        &sence,
        setting.size,
    );
    #[cfg(debug_assertions)]
    path.render_process_debug(
        &setting.name,
        setting.sample_num,
        &sence,
        setting.size,
        Default::default(),
    );
}
