#[cfg(test)]
pub mod test{
    use crate::pbrt_core::load::GltfLoad;
    use crate::pbrt_core::tool::log::log_init;
    #[test]
    fn test_load(){
        log_init();
        GltfLoad::load("./object/cube/yuans.gltf");
        // khr_lights_punctual
    }
}
