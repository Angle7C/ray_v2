
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles};

use crate::pbrt_core::tool::{func::{self, transform_common}, Bound, InteractionCommon, Shading};

use super::ShapeAble;
#[derive(Debug)]
pub struct Rectangle {
    pub obj_to_world: Mat4,
}
impl Rectangle {
    pub fn new(obj_to_world: Mat4) -> Self {
        Self {
            obj_to_world,
        }
    }
}
impl ShapeAble for Rectangle {
    fn bound(&self)->Bound<3> {
        Bound::<3>::new(Vec3::ZERO, Vec3::ONE)
    }
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let min = self.obj_to_world.transform_point3(Vec3::ZERO); 
        let max = self.obj_to_world.transform_point3(Vec3::ONE);
        Bound::<3>::new(min, max)
    }
    fn area(&self)->f32 {
        let p1 = self.obj_to_world.transform_vector3(Vec3::X);
        let p2 = self.obj_to_world.transform_vector3(Vec3::Y);
        p1.cross(p2).length()
    }
    
    fn sample(&self,u:Vec2,pdf:&mut f32)->InteractionCommon{
        
        let mut common=InteractionCommon::default();
        //计算点
        common.p= u.extend(0.0);
        //计算法线
        common.normal = Vec3::Z;
        common.w0=Vec3::Z;
        //计算PDF
        *pdf=1.0/self.area();
        func::transform_common(self.obj_to_world, common)
    }

    fn intersect(&self, ray: crate::pbrt_core::tool::RayDiff) -> Option<InteractionCommon> {
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir).normalize();
        let o=self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let t = -o.z/dir.z;
        let p= o+t*dir;
        if p.x<0.0||p.x>1.0||p.y<0.0||p.y>1.0{
            None
        }else{
            let mut common = InteractionCommon::default();
            common.p=p;
            common.normal=Vec3::Z;
            common.w0=-dir;
            common.time=t;
            common.uv=p.xy();
            common.shading=Shading::default();
            common.shading.n=Vec3::Z;
            common.shading.dpdu=Vec3::X;
            common.shading.dpdv=Vec3::Y;
            common=transform_common(self.obj_to_world, common);
            Some(common)
        }
    }

    fn intersect_p(&self, ray: &crate::pbrt_core::tool::RayDiff) -> bool {
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir).normalize();
        let o=self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let t = -o.z/dir.z;
        return (t<ray.o.t_min||t>ray.o.t_max)&&t>0.0
       
    }

    fn sample_with_ref_point(&self,_common:&InteractionCommon,_u:Vec2,_pdf:&mut f32)->InteractionCommon {
        todo!()
    }

    fn pdf_with_ref_point(&self,_common:&InteractionCommon,_w_in:&Vec3)->f32 {
        todo!()
    }
    fn obj_to_world(&self)->Mat4 {
        self.obj_to_world
    }
}
