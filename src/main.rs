use std::{fs::File, path::Path};
use std::io::Read;
use pbrt_core::tool::{log::log_init, build::Context};
use crate::pbrt_core::load::{FactoryType, LoadSceneFactory};

pub mod pbrt_core;

mod test;

pub fn main() {
    log_init();

    let factory = LoadSceneFactory::create_factory(FactoryType::TomlFactory);
    let path=Path::new("./file/setting.toml");
    let file = File::open(path).expect("打开文件失败");
    let data = file.bytes().map(|i|i.unwrap()).collect::<Vec<_>>();
    let context= factory.load(&data).expect("加载场景失败");
    context.render();

}