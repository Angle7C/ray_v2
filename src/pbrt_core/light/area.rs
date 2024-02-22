
use glam::{Vec2, Vec3};



use crate::pbrt_core::light::LightType;
use crate::pbrt_core::primitive::shape::ShapeAble;
use crate::pbrt_core::tool::color::Color;

use crate::pbrt_core::{
    primitive::{Primitive},
    tool::{InteractionCommon, RayDiff, Visibility},
};

use super::LightAble;

#[derive(Debug)]
pub struct DiffuseAreaLight {
    emit: Vec3,
}

impl DiffuseAreaLight {
    pub fn new(emit: Vec3) -> Self {
        Self {emit}
    }
}
impl LightAble for DiffuseAreaLight {
    fn sample_li(&self,surface:&InteractionCommon,
            light_face:&mut InteractionCommon,
            shape:Option<&dyn ShapeAble>,
            u:Vec2,
            wi:&mut Vec3,
            pdf:&mut f32,
            vis:&mut Visibility
            )->Color {
                let mut ans=Color::ZERO;
                if let Some(shape) =  shape {
                     *light_face=shape.sample(u, pdf);
                    ans=if(pdf.abs()<f32::EPSILON)||(surface.p-light_face.p).length_squared().abs()<f32::EPSILON{
                        *pdf=0.0;
                        Color::ZERO
                     }else{
                        *wi=(light_face.p-surface.p).normalize();
                        *vis=Visibility{
                            a:*light_face,
                            b:*surface
                        };
                        self.li(light_face, wi)
                     }
                }
                ans
    }
    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color {
        if inter.normal.dot(*w) > 0.0 {
            self.emit.into()
        } else {
            Color::ZERO
        }
    }
    fn pdf_li(&self,face:&InteractionCommon,w_in:&Vec3,shape: Option<&dyn ShapeAble>,)->f32 {
        shape
        .and_then(|item|{
            if item.intersect_p(&face.spawn_ray(w_in)) {
                Some(1.0/item.area())
            }else{None}
       }).unwrap_or(0.0)
    }
    fn get_type(&self) -> LightType {
        LightType::Area
    }
    fn le(&self, ray: &RayDiff,shape:Option<&dyn ShapeAble>) -> Color {
        let dir = shape.map(|item| item.obj_to_world().inverse().transform_vector3(ray.o.dir).normalize());
        match dir{
            Some(value) if value.dot(Vec3::Z) >0.0 => self.emit.into(),
            _ =>Color::ZERO,
        }

    }

    fn pdf_le(&self,_ray:&RayDiff,_normal:Vec3,_pdf_pos:&mut f32,_pdf_dir:&mut f32) {
        todo!()
    }

    fn power(&self)->Color{
        Color::ONE
    }

    fn sample_le(&self,_u1:Vec2,_u2:Vec2,_t:f32)->Option<bvh::ray::Ray>{
        None
    }
}
