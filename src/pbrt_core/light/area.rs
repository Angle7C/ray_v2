use std::sync::Arc;
use glam::{Vec2, Vec3};
use log::info;

use crate::pbrt_core::light::LightType;
use crate::pbrt_core::primitive::shape::ShapeAble;
use crate::pbrt_core::tool::color::Color;
use crate::pbrt_core::{
    primitive::{Primitive},
    tool::{Bound, InteractionCommon, RayDiff, SurfaceInteraction, Visibility},
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
    fn get_samples(&self) -> usize {
        32
    }
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
                        *wi=(surface.p-light_face.p).normalize();
                        *vis=Visibility{
                            a:*light_face,
                            b:*surface
                        };
                        self.li(&light_face, wi)
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
    fn pdf_li(&self,face:&InteractionCommon,w_in:&Vec3)->f32 {
        todo!()
    }
    fn get_type(&self) -> LightType {
        LightType::Area
    }
    fn le(&self, ray: &RayDiff) -> Color {
        unimplemented!()
    }

    fn pdf_le(&self,ray:&RayDiff,normal:Vec3,pdf_pos:&mut f32,pdf_dir:&mut f32) {
        todo!()
    }

    fn power(&self)->Color{
        Color::ONE
    }

    fn sample_le(&self,u1:Vec2,u2:Vec2,t:f32)->Option<bvh::ray::Ray>{
        None
    }
}
