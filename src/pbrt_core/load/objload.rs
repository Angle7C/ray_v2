
use anyhow::Ok;
use glam::{UVec3, Vec2, Vec3};
use obj::Obj;

use crate::pbrt_core::primitive::mesh::Mesh;

pub struct ObjLoad;

impl ObjLoad {
    pub fn load<'a>(path: &str) -> anyhow::Result<(Mesh, Vec<Vec<UVec3>>)> {
        let obj = Obj::load(path)?;
        let data = &obj.data;
        let point = data
            .position
            .iter()
            .map(|item| Vec3::new(item[0], item[1], item[2]))
            .collect::<Vec<_>>();
        let normal = data
            .normal
            .iter()
            .map(|item| Vec3::new(item[0], item[1], item[2]))
            .collect::<Vec<_>>();
        let tex = data
            .texture
            .iter()
            .map(|item| Vec2::new(item[0], item[1]))
            .collect::<Vec<_>>();
        let mesh = Mesh::new(point, normal, tex, vec![]);
        let mut index_vec = { vec![vec![], vec![], vec![]] };
        for obj in &data.objects {
            let iter = obj.groups.iter().flat_map(|group| group.polys.iter());
            let mut pos_index = UVec3::new(0, 0, 0);
            let mut tex_index = UVec3::new(0, 0, 0);
            let mut norm_index = UVec3::new(0, 0, 0);
            for item in iter {
                for (index, i) in item.0.iter().enumerate() {
                    pos_index[index] = i.0 as u32;
                    tex_index[index] = i.1.unwrap() as u32;
                    norm_index[index] = i.2.unwrap() as u32;
                }
                index_vec[0].push(pos_index);
                index_vec[1].push(tex_index);
                index_vec[2].push(norm_index);
            }
        }
        Ok((mesh, index_vec))
    }
}
