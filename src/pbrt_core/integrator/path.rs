use glam::Vec3A;
use rand::random;
use serde::de;

use crate::pbrt_core::{sampler::Sampler, tool::{sence::Sence, ray::Ray, film::Film, color::Color}, bxdf::BxDFType};

use super::{Integrator, uniform_sample_one_light};

pub struct PathIntegrator {
    q: f32,
    dept: usize,
}
impl PathIntegrator {
    pub fn new(q:f32,dept:usize)->PathIntegrator{
        Self { q, dept }
    }
}
impl Integrator for PathIntegrator {
    fn is_next(&self, dept: &mut usize) -> Option<f32> {
        *dept += 1;
        if *dept <= self.dept {
            Some(1.0)
        } else {
            let q: f32 = random();
            if q < self.q {
                Some(1.0 - q)
            } else {
                None
            }
        }
    }
    fn fi(&self,ray:Ray,sence:&Sence,mut sampler: Sampler) -> crate::pbrt_core::tool::color::Color {
        let mut ld=Color::ZERO;
        let mut dept=0;
        let mut beta=Color::ZERO;
        let mut sample_type=0;
        let mut ray=ray;
        while let Some(q)=self.is_next(&mut dept){
            if let Some(ref mut item) =sence.interacect(&ray) {
                if dept==1{
                    
                }else{

                }
                item.compute_scattering(&ray,crate::pbrt_core::light::TransportMode::Radiance);
                if let Some(ref bsdf)=item.bsdf{
                    ld+=beta*uniform_sample_one_light(&item, sence, sampler.clone());
                    let wo=-ray.dir;
                    let mut wi:Vec3A=Default::default();
                    let mut pdf=0.0;
                    let f = bsdf.sample_f(wo, &mut wi, &sampler.sample_2d(),&mut pdf, BxDFType::All.into(),&mut sample_type)
                    * wi.dot(item.shading.n).abs()/pdf;
                    beta=f*beta;
                    ray=Ray::new_default(item.common.wo, wi);
                }
                beta=beta/q;
            }else{
                return ld;
            }
        };
        ld
      
    }
}
