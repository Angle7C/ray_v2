
pub mod test{
    #[cfg(test)]
    use crate::pbrt_core::load::GltfLoad;
    #[test]
    fn test_load(){
        GltfLoad::load("./object/cube/cube.gltf");
    }
}
