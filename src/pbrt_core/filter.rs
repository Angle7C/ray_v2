use glam::Vec2;
#[derive(Debug,Default)]
pub enum Filter{
    #[default]
    Repeat
}
impl Filter{
    #[inline]
    pub fn filter_uv(&self,uv:&Vec2)->Vec2{
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
            uv.y.abs()-uv.y.floor()
        }else{
            uv.y
        };
        Vec2 { x, y }
    }
}