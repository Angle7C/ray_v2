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
        let path=Path::new("./file/setting.toml");
        let mut file = File::open(path).expect("读取配置文件失败");
        let mut buf:String=String::new();
        file.read_to_string(&mut buf).unwrap();
        let a = Vec2::new(1.0, 1.0);
        

        let load = toml::from_str::<MyLoad>(&buf).unwrap();
        print!("{:?}",load);
    }
}
