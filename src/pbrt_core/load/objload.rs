use std::{error::Error, any};

use glam::{Mat4, UVec3, Vec3, Vec2};
use obj::Obj;

use crate::pbrt_core::{
    material::Material,
    primitive::{shape::triangle::Triangle, Primitive, mesh::Mesh},
    tool::sence::Sence, texture,
};

pub struct ObjLoad;

impl ObjLoad {
    pub fn load<'a>(
        path: &str,
        trans: Mat4,
        material: Option<&'a Box<dyn Material + 'a>>,
    ) ->anyhow::Result<(Mesh, Vec<usize>)> {
        let obj = Obj::load(path)?;
        let data = &obj.data;
        let pos=data.position.iter().map(|item|
            Vec3::new(item[0], item[1], item[2])
        ).collect::<Vec<_>>();
        let normal = data.normal.iter().map(|item|
            Vec3::new(item[0], item[1], item[2])
        ).collect::<Vec<_>>();
        let tex=data.texture.iter().map(|item|
            Vec2::new(item[0], item[1])
        ).collect::<Vec<_>>();
        let _ = obj.data
            .objects
            .iter()
            .map(|obj| {
                obj.groups.iter().map(|group| {
                    // 一个面
                    group.polys.iter().map(|ploy| {
                        //一个点的参数
                        for item in &ploy.0 {
                            let pos = item.0;
                            let tex = item.1.unwrap();
                            let norm = item.2.unwrap();
                            let pos_i = data.position.get(pos);
                            let tex_i = data.texture.get(tex);
                            let norm_i = data.normal.get(norm);
                            if let (Some(pos_i), Some(tex_i), Some(norm_i)) = (pos_i, tex_i, norm_i)
                            {

                            }
                        }
                        
                    })
                })
            })
            .collect::<Vec<_>>();
        unimplemented!()
    }
}
