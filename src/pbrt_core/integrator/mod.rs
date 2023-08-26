use glam::f64::DVec3;
use image::Rgb;

use self::path::PathIntegrator;

pub mod path;

pub enum Integrator{
    Path(Box<PathIntegrator>)
}

pub fn to_color(color:DVec3,ssp:f64)->Rgb<u8>{
        let vec = (color / ssp).powf(0.5);
        let rgb = vec * 255.0;
        let color= Rgb([
            rgb.x.clamp(0.0, 255.0) as u8,
            rgb.y.clamp(0.0, 255.0) as u8,
            rgb.z.clamp(0.0, 255.0) as u8,
        ]);
        color
}