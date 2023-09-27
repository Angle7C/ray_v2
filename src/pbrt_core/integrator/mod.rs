
use std::thread;

use glam::{Vec2, Vec3A};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{bxdf::BxDFType, light::LightType, tool::vistest::VisibilityTester};

use self::path::PathIntegrator;

use super::{
    light::LightAble,
    sampler::{Sampler, self},
    tool::{color::Color, interaction::SurfaceInteraction, ray::Ray, sence::Sence, content::{Content, Setting}, film::Film}, camera::{Camera, CameraSample},
};

pub mod path;
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]

pub enum LightStartegy {
    SampleAll,
    SampleOne,
}
pub trait Integrator {
    fn fi(&self, ray: Ray, sence: &Sence,sampler: Sampler) -> Color;
    fn is_next(&self, dept: &mut usize) -> Option<f32>;
}
pub enum SampleIntegrator{
    Path(PathIntegrator)
}
impl Integrator for SampleIntegrator{
    fn fi(&self, ray: Ray, sence: &Sence,mut sampler: Sampler) -> Color {
        todo!()
    }

    fn is_next(&self, dept: &mut usize) -> Option<f32> {
        todo!()
    }
}
impl SampleIntegrator{
    pub fn render<'a>(self,sence:Sence<'a>,film:Film,setting:Setting){
        let camera=sence.camera;
        let sampler=Sampler::new(setting.num as usize);
        thread::scope(|scope|{
            for _ in 0..setting.core_num{
                scope.spawn(self.render_core(&camera, &sence, &film, sampler.clone(), setting.num));
            }
        });
    }
    pub fn render_core<'a,'b>(&'a self,camera:&'a Camera,sence:&'a Sence,film:&'a Film,sampler:Sampler,num:u32)->impl FnOnce()+'b
    where 'a:'b{
        move ||{
            let mut sampler=sampler.clone();
            while let Some(item) = film.iter(){
                let mut ans=Color::ZERO;
                for point in item{
                       for _ in 0..num{
                            let sample=CameraSample::new(point.x, point.y, &mut sampler);
                            let ray = camera.generate_ray(sample);
                            ans+=self.fi(ray, sence, sampler.clone());
                       } 
                }
                ans=ans/num as f32
            }
        }
    }
}










fn uniform_sample_one_light<'a>(
    inter: &'a SurfaceInteraction,
    sence: &'a Sence<'a>,
    mut sampler: Sampler,
) -> Color {
    if sence.light.len() == 0 {
        return Color::ZERO;
    };
    //随机选择一个light
    let num = sampler.rand.gen_range(0..sence.light.len());
    let light = &sence.light[num];
    estimate_direct(
        inter,
        &sampler.sample_2d(),
        light,
        &sampler.sample_2d(),
        sence,
        &mut sampler,
        true,
    )/sence.light.len() as f32
}

fn estimate_direct<'a>(
    inter: &SurfaceInteraction,
    u_scatter: &Vec2,
    light: &'a Box<dyn LightAble+'a>,
    u_light: &Vec2,
    sence: &'a Sence<'a>,
    sampler: &mut Sampler,
    specular: bool,
) -> Color {
    let bxdf_type = if specular {
        BxDFType::All as u32
    } else {
        !(BxDFType::All as u32 & BxDFType::Specular as u32)
    };
    let mut ld = Color::ZERO;
    let mut wi = Vec3A::default();
    let mut vis = VisibilityTester::default();
    let mut light_pdf = 0.0;
    let mut scatter_pdf = 0.0;
    let mut light_common = Default::default();
    let mut li = light.sample_li(
        &inter.common,
        &mut light_common,
        u_light,
        &mut wi,
        &mut light_pdf,
        &mut vis,
    );
    if !li.is_black() && light_pdf > f32::EPSILON {
        if let Some(ref bsdf) = inter.bsdf {
            let f = bsdf.f(inter.common.wo, wi, bxdf_type) * wi.dot(inter.shading.n).abs();
            scatter_pdf = bsdf.pdf(&inter.common.wo, &wi, bxdf_type);
            if !f.is_black() {
                if !vis.unoccluded(&sence) {
                    li = Color::ZERO;
                }
            }
            if !li.is_black() {
                if LightType::is_delta(light.get_type()) {
                    ld += f * li / light_pdf;
                } else {
                    let weight = 0.5;
                    ld += f * li * weight / light_pdf;
                }
            }
        }
    }
    if !LightType::is_delta(light.get_type()) {
        let mut sampler_type = 0_u32;
        let mut sampler_specular = false;
        if let Some(ref bsdf) = inter.bsdf {
            let f = bsdf.sample_f(
                inter.common.wo,
                &mut wi,
                &sampler.sample_2d(),
                &mut scatter_pdf,
                bxdf_type,
                &mut sampler_type,
            ) * wi.dot(inter.common.n).abs();
            sampler_specular = (sampler_type & BxDFType::Specular as u32) != 0;
            if !f.is_black() {
                let weight = if !sampler_specular {
                    light_pdf = light.pdf_li(inter, wi);
                    if light_pdf < f32::EPSILON {
                        return ld;
                    }
                    0.5
                } else {
                    1.0
                };
                let ray = Ray::new_default(inter.common.p, wi);
                if let Some(ref light_item) = sence.interacect(&ray) {
                    li = light_item.le(&-ray.dir);
                } else {
                    li = light.le(&ray);
                }
                if li.is_black() {
                    ld += f * li * weight / scatter_pdf;
                }
            }
        }
    }
    ld
}
