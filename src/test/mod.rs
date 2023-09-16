#[cfg(test)]
pub mod test{

    use std::{path::Path, fs::File, io::Read};

    use glam::Vec2;


    use crate::pbrt_core::{tool::log::log_init, load::myload::{MyLoad, ShapeToml, MaterialToml}};
    #[test]
    fn test_load(){
        log_init();
        // khr_lights_punctual
    }
    #[test]
    fn load_test(){
        // let mut load=MyLoad::default();
        let m = MaterialToml::Matte { kd: 0, sigma: 0 };
        let str = toml::to_string(&m).unwrap();
        print!("{:?}",str);
    }
}
