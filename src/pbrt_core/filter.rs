use glam::{DVec3, DVec2};
#[derive(Debug,Default)]
pub enum Filter{
    #[default]
    Repeat
}
impl Filter{
    #[inline]
    pub fn filter_uv(&self,uv:&DVec2)->DVec2{
        let x=if uv.x<0.0{
            uv.x.floor().abs()+uv.x
        }else if uv.x>=1.0{
            uv.x.abs()-uv.x.floor()
        }else{
            uv.x
        };
        let y=if uv.y<0.0{
            uv.y.floor().abs()+uv.y
        }else if uv.y>=0.0{
            uv.y.abs()-uv.x.floor()
        }else{
            uv.y
        };
        DVec2 { x, y }
    }
}