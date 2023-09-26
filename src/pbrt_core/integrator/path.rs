use glam::Vec3A;
use rand::random;
use serde::de;

use crate::pbrt_core::{sampler::Sampler, tool::{sence::Sence, ray::Ray, film::Film, color::Color}, bxdf::BxDFType};

use super::Integrator;

pub struct PathIntegrator {
    q: f32,
    dept: usize,
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
    fn fi(&self,ray:Ray,sence:&Sence,sampler:&mut Sampler) -> crate::pbrt_core::tool::color::Color {
        let mut ld=Color::ZERO;
        let mut dept=0;
        let mut beta=Color::ZERO;
        while let Some(q)=self.is_next(&mut dept){
            if let Some(ref mut item) =sence.interacect(&ray) {
                if dept==1{
                    
                }else{

                }
                item.compute_scattering(&ray,crate::pbrt_core::light::TransportMode::Radiance);
                if let Some(ref bsdf)=item.bsdf{
                    let wo=-ray.dir;
                    let mut wi:Vec3A=Default::default();
                    let mut pdf=0.0;
                    let f = bsdf.sample_f(wo, &mut wi, &mut pdf, BxDFType::All.into())
                    * wi.dot(item.shading.n).abs()/pdf;
                    beta=f*beta;
                }
                beta=beta/q;
            }   

        }
        unimplemented!()
    }
}
