use std::error::Error;

use obj::Obj;

use crate::pbrt_core::{tool::sence::Sence, primitive::Primitive};

pub struct ObjLoad;

impl ObjLoad {
    pub fn load(path: &str) -> anyhow::Result<Vec<Box<dyn Primitive>>> {
        let obj = Obj::load(path)?;
    
        unimplemented!()
    }
}
