use glam::{Affine3A, Vec3A, Vec2};

use crate::pbrt_core::{material::MaterialAble, tool::{interaction::InteractionCommon, bound::Bound}, primitives::Primitive};

use super::ShapeAble;



pub struct Rectangle<'a> {
    pub obj_to_world: Affine3A,
    material: Option<&'a dyn MaterialAble>,
}
impl<'a> Rectangle<'a> {
    pub fn new(obj_to_world: Affine3A, material: Option<&'a dyn MaterialAble>) -> Self {
        Self {
            obj_to_world,
            material,
        }
    }
    pub fn get_area(&self) -> f32 {
        let p1 = self.obj_to_world.transform_vector3a(Vec3A::X);
        let p2 = self.obj_to_world.transform_vector3a(Vec3A::Y);
        p1.cross(p2).length()
        // DMat2::from_cols(self.obj_to_world.x_axis.xy(), self.obj_to_world.y_axis.xy()).determinant()
    }
    pub fn sample_interaction(&self, sampler_point: Vec2) -> InteractionCommon {
        let p = self
            .obj_to_world
            .transform_point3a(sampler_point.extend(0.0).into());
        let mut commom = InteractionCommon {
            ..Default::default()
        };
        commom.p = p;
        commom.n = self.obj_to_world.transform_vector3a(Vec3A::Z);
        commom
    }
}
impl<'a>  ShapeAble for Rectangle<'a> {
    fn world_bound(&self)->Bound<3> {
        todo!()
    }

    fn area(&self)->f32 {
        let p1 = self.obj_to_world.transform_vector3a(Vec3A::X);
        let p2 = self.obj_to_world.transform_vector3a(Vec3A::Y);
        p1.cross(p2).length()
    }
}
impl<'a> Primitive for Rectangle<'a> {
    fn intersect(&self,ray:&crate::pbrt_core::tool::ray::Ray) -> Option<crate::pbrt_core::tool::interaction::SurfaceInteraction> {
        let o = self.obj_to_world.inverse().transform_point3a(ray.o);
        let dir = self.obj_to_world.inverse().transform_vector3a(ray.dir);
        let t = -o.z / dir.z;
        let p = o + dir * t;
        if p.x < 0.0 || p.x > 1.0 {
            return None;
        }
        if p.y < 0.0 || p.y > 1.0 {
            return None;
        }
        let p = self.obj_to_world.transform_point3a(p).normalize();
        let n = self.obj_to_world.transform_vector3a(Vec3A::Z).normalize();
        let dpdu = self.obj_to_world.transform_point3a(Vec3A::X).normalize();
        let dpdv = self.obj_to_world.transform_point3a(Vec3A::Y).normalize();
        // let surface = SurfaceInteraction::new(
        //     p,
        //     p.truncate(),
        //     n,
        //     ray.dir,
        //     dpdu,
        //     dpdv,
        //     Vec3A::ZERO,
        //     Vec3A::ZERO,
        //     t,
        //     Some(self),
        //     None,
        // );
        // Some(surface)
        unimplemented!()
    }
    fn world_bound(&self) -> crate::pbrt_core::tool::bound::Bound<3> {
        let min = self.obj_to_world.transform_point3a(Vec3A::ZERO) - Vec3A::splat(0.003);
        let max = self.obj_to_world.transform_point3a(Vec3A::ONE) + Vec3A::splat(0.003);
        Bound::<3>::new(min, max)
    }
}
