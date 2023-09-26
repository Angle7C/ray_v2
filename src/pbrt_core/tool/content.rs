use std::{fs::File, io::{BufReader, Read}};

use crate::pbrt_core::load::TomlLoad;

use super::sence::Sence;


pub struct Setting{
    name:String,
    num:u32,
    core_num:u32,
}

pub struct Content<'a>{
    sence:Sence<'a>,
    setting:Setting
}
impl<'a> Content<'a>{
    pub fn new(path:&str)->Self{
        let mut file = File::open(path).expect("读取文件失败");
        let mut buf=String::new();
        file.read_to_string(&mut buf).expect("读取文件失败");

        let load = toml::from_str::<TomlLoad>(&buf).unwrap();
        unimplemented!()

    }
}