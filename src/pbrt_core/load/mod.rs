use crate::pbrt_core::tool::build::Context;
use crate::pbrt_core::tool::sence::Scene;

use self::toml_load::TomlLoader;

pub mod toml_load;

// 加载器接口
pub trait LoadSceneAble{
    fn load(&self,data:&[u8])->anyhow::Result<Context>;
}


// 工厂创建结构
pub struct  LoadSceneFactory;

//加载器类型
pub enum FactoryType{
    TomlFactory
}
impl LoadSceneFactory{
    pub fn create_factory(factory_type: FactoryType)->Box<dyn LoadSceneAble>{
        match factory_type {
            FactoryType::TomlFactory=>Box::new(TomlLoader)
        }
    }
}
