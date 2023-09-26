use serde::{Deserialize, Serialize};

use super::{
    sampler::Sampler,
    tool::{color::Color, ray::Ray, sence::Sence},
};

pub mod path;
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]

pub enum LightStartegy{
    SampleAll,
    SampleOne
}
pub trait Integrator {
    fn fi(&self, ray: Ray, sence: &Sence, sampler: &mut Sampler) -> Color;
    fn is_next(&self, dept: &mut usize) -> Option<f32>;
}
