use std::{fs::File, path::Path};
use std::io::Read;
use pbrt_core::tool::log::log_init;
use crate::pbrt_core::load::{FactoryType, LoadSceneFactory};

pub mod pbrt_core;

#[cfg(test)]
mod test;

pub fn main() {
    //日志初始化
    log_init();
    //工厂创建
    let factory = LoadSceneFactory::create_factory(FactoryType::TomlFactory);
    //读取文件并加载数据字节
    let path=Path::new("./file/setting.toml");
    let file = File::open(path).expect("打开文件失败");
    let data = file.bytes().map(|i|i.unwrap()).collect::<Vec<_>>();
    //初始化上下文
    let context= factory.load(&data).expect("加载场景失败");
    //启用渲染
    context.render();

}