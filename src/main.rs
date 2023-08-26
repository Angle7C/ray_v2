
use pbrt_core::tool::setting::Build;

#[allow(unused,dead_code)]
pub mod pbrt_core;

mod test;

pub fn main(){
    let (sence,path) = Build::build("./sence/sence.json");
    #[cfg(not(debug_assertions))]
    path.render_process("test", 6, &sence);
    #[cfg(debug_assertions)]
    path.render_process_debug("test", 6,  &sence);

}