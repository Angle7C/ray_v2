use super::{phase_h_g, PhaseFunction};

pub struct HenyeyGreenstein{
    g:f32,
}
impl HenyeyGreenstein{
    pub fn new(g:f32)->Self{
        Self{g}
    }
}
impl PhaseFunction for HenyeyGreenstein{
    fn p(&self,wo:&glam::Vec3,wi:glam::Vec3)->f32 {
        phase_h_g(wo.dot(wi), self.g)
    }

    fn sample_p(&self,wo:&glam::Vec3,wi:&mut glam::Vec3,u:glam::Vec2)->f32 {
        todo!()
    }
}
