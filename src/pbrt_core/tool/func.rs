use glam::DVec3;

use crate::pbrt_core::bxdf::func::{sin_phi, cos_phi, cos_theta};

pub fn vec3_coordinate_system(v1:DVec3,v2:&mut DVec3,v3:&mut DVec3){
    if v1.x.abs()>v1.y.abs(){
        *v2=DVec3::new(-v1.z, 0.0, v1.x)
        /(v1.x*v1.x+v1.z*v1.z).sqrt();
    }else{
        *v2=DVec3::new(0.0, v1.z, -v1.y)
        /(v1.y*v1.y+v1.z+v1.z).sqrt();
    }
    *v3=v1.cross(*v2);
}

pub fn spherical_direction(sin_theta: f64, cos_theta: f64, phi: f64) -> DVec3 {
    DVec3 {
        x: sin_theta * phi.cos(),
        y: sin_theta * phi.sin(),
        z: cos_theta,
    }
}

pub fn trowbridge_reitz_sample(
    wi: &DVec3,
    alpha_x: f64,
    alpha_y: f64,
    u1: f64,
    u2: f64,
) -> DVec3 {
    // 1. stretch wi
    let wi_stretched: DVec3 = DVec3 {
        x: alpha_x * wi.x,
        y: alpha_y * wi.y,
        z: wi.z,
    }
    .normalize();

    // 2. simulate P22_{wi}(x_slope, y_slope, 1, 1)
    let mut slope_x = 0.0;
    let mut slope_y = 0.0;
    trowbridge_reitz_sample_11(crate::pbrt_core::bxdf::func::cos_theta(&wi_stretched), u1, u2, &mut slope_x, &mut slope_y);

    // 3. rotate
    let tmp: f64 = cos_theta(&wi_stretched) * slope_x - sin_phi(&wi_stretched) * slope_y;
    slope_y = sin_phi(&wi_stretched) * slope_x + cos_phi(&wi_stretched) * slope_y;
    slope_x = tmp;

    // 4. unstretch
    slope_x *= alpha_x;
    slope_y *= alpha_y;

    // 5. compute normal
    DVec3 {
        x: -slope_x,
        y: -slope_y,
        z: 1.0,
    }
    .normalize()
}
pub fn trowbridge_reitz_sample_11(
    cos_theta: f64,
    u1: f64,
    u2: f64,
    slope_x: &mut f64,
    slope_y: &mut f64,
) {
    // special case (normal incidence)
    if cos_theta > 0.9999 {
        let r = (u1 / (1.0 - u1)).sqrt();
        let phi = std::f64::consts::TAU * u2;
        *slope_x = r * phi.cos();
        *slope_y = r * phi.sin();
        return;
    }

    let sin_theta=0.0_f64
        .max(1.0  - cos_theta * cos_theta)
        .sqrt();
    let tan_theta = sin_theta / cos_theta;
    let a= 1.0 / tan_theta;
    let g1 = 2.0 / (1.0 + (1.0 + 1.0 / (a * a)).sqrt());

    // sample slope_x
    let a = 2.0 * u1 / g1 - 1.0;
    let mut tmp= 1.0 / (a * a - 1.0);
    if tmp > 1e10 {
        tmp = 1e10;
    }
    let b= tan_theta;
    let d = (b * b * tmp * tmp - (a * a - b * b) * tmp)
        .max(0.0)
        .sqrt();
    let slope_x_1= b * tmp - d;
    let slope_x_2 = b * tmp + d;
    if a < 0.0 || slope_x_2 > 1.0 / tan_theta {
        *slope_x = slope_x_1;
    } else {
        *slope_x = slope_x_2;
    }

    // sample slope_y
    let s;
    let new_u2 = if u2 > 0.5 {
        s = 1.0;
        2.0 * (u2 - 0.5)
    } else {
        s = -1.0;
        2.0 * (0.5 - u2)
    };
    let z = (new_u2 * (new_u2 * (new_u2 * 0.27385 - 0.73369) + 0.46341))
        / (new_u2 * (new_u2 * (new_u2 * 0.093_073 + 0.309_420) - 1.0) + 0.597_999);
    *slope_y = s * z * (1.0 + *slope_x * *slope_x).sqrt();

    assert!(!(*slope_y).is_infinite());
    assert!(!(*slope_y).is_nan());
}