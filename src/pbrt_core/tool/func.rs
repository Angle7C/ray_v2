use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use glam::{Vec2, Vec3};

pub fn cosine_sample_hemisphere(u: Vec2) -> Vec3 {
    let d = concentric_sample_disk(u);
    let z = 0.0_f32.max(1.0 - d.length_squared()).sqrt();
    d.extend(z)
}

pub fn concentric_sample_disk(u: Vec2) -> Vec2 {
    let offset = u * 2.0 - Vec2::ONE;
    if offset.x == 0.0 && offset.y == 0.0 {
        return Vec2::ZERO;
    }
    let theta;
    let r;
    if offset.x.abs() > offset.y.abs() {
        r = offset.x;
        theta = FRAC_PI_4 * (offset.y / offset.x);
    } else {
        r = offset.y;
        theta = FRAC_PI_2 - FRAC_PI_4 * (offset.x / offset.y)
    }
    Vec2 {
        x: theta.cos(),
        y: theta.sin(),
    } * r
}
pub fn quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let det=b*b-4.0*a*c;
    if det<0.0{
        None
    }else{
        let t1=(-b+det.sqrt())/(2.0*a);
        let t2=(-b-det.sqrt())/(2.0*a);
        Some((t1,t2))
    }
}
