use glam::DVec3;

use crate::pbrt_core::{sampler::Sampler, tool::{sence::Sence, Ray, RayDiff}, primitive::Primitive, bxdf::TransportMode};

pub struct DirectIntegrator{
    max_depth:u32,
    strategy:LightStartegy,
    sample:Sampler
}
pub enum LightStartegy{
    UniformAll,
    UniformOne
}
impl DirectIntegrator{
    pub fn li(&self,sence:&Sence,ray:RayDiff)->DVec3{
        let mode=TransportMode::Radiance;
        let mut l=DVec3::ZERO;
        if let Some(mut surface)=sence.interacect(ray){
            surface.compute_scattering(ray, mode);
            if let Some(ref bsdf)=surface.bsdf{
                let w_out=surface.common.w0;
                return sence.uniform_sample_one_light(&surface, &mut self.sample.clone(), false);        
                // match self.strategy{
                //     LightStartegy::UniformAll=>{
                //         l+=
                //     },
                //     LightStartegy::UniformOne=>{   
                //     }
                // };
            }
            
        }
        DVec3::ZERO
    }
}