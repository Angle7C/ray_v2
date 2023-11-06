use glam::{Mat4, Vec3};

use crate::pbrt_core::bxdf::func::{cos_phi, cos_theta, sin_phi};

use super::{InteractionCommon, Shading, SurfaceInteraction};

pub fn vec3_coordinate_system(v1: Vec3, v2: &mut Vec3, v3: &mut Vec3) {
    if v1.x.abs() > v1.y.abs() {
        *v2 = Vec3::new(-v1.z, 0.0, v1.x) / (v1.x * v1.x + v1.z * v1.z).sqrt();
    } else {
        *v2 = Vec3::new(0.0, v1.z, -v1.y) / (v1.y * v1.y + v1.z + v1.z).sqrt();
    }
    *v3 = v1.cross(*v2);
}

pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vec3 {
    Vec3 {
        x: sin_theta * phi.cos(),
        y: sin_theta * phi.sin(),
        z: cos_theta,
    }
}

pub fn trowbridge_reitz_sample(wi: &Vec3, alpha_x: f32, alpha_y: f32, u1: f32, u2: f32) -> Vec3 {
    // 1. stretch wi
    let wi_stretched: Vec3 = Vec3 {
        x: alpha_x * wi.x,
        y: alpha_y * wi.y,
        z: wi.z,
    }
    .normalize();

    // 2. simulate P22_{wi}(x_slope, y_slope, 1, 1)
    let mut slope_x = 0.0;
    let mut slope_y = 0.0;
    trowbridge_reitz_sample_11(
        crate::pbrt_core::bxdf::func::cos_theta(&wi_stretched),
        u1,
        u2,
        &mut slope_x,
        &mut slope_y,
    );

    // 3. rotate
    let tmp: f32 = cos_theta(&wi_stretched) * slope_x - sin_phi(&wi_stretched) * slope_y;
    slope_y = sin_phi(&wi_stretched) * slope_x + cos_phi(&wi_stretched) * slope_y;
    slope_x = tmp;

    // 4. unstretch
    slope_x *= alpha_x;
    slope_y *= alpha_y;

    // 5. compute normal
    Vec3 {
        x: -slope_x,
        y: -slope_y,
        z: 1.0,
    }
    .normalize()
}
pub fn trowbridge_reitz_sample_11(
    cos_theta: f32,
    u1: f32,
    u2: f32,
    slope_x: &mut f32,
    slope_y: &mut f32,
) {
    // special case (normal incidence)
    if cos_theta > 0.9999 {
        let r = (u1 / (1.0 - u1)).sqrt();
        let phi = std::f32::consts::TAU * u2;
        *slope_x = r * phi.cos();
        *slope_y = r * phi.sin();
        return;
    }

    let sin_theta = 0.0_f32.max(1.0 - cos_theta * cos_theta).sqrt();
    let tan_theta = sin_theta / cos_theta;
    let a = 1.0 / tan_theta;
    let g1 = 2.0 / (1.0 + (1.0 + 1.0 / (a * a)).sqrt());

    // sample slope_x
    let a = 2.0 * u1 / g1 - 1.0;
    let mut tmp = 1.0 / (a * a - 1.0);
    if tmp > 1e10 {
        tmp = 1e10;
    }
    let b = tan_theta;
    let d = (b * b * tmp * tmp - (a * a - b * b) * tmp).max(0.0).sqrt();
    let slope_x_1 = b * tmp - d;
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

pub fn same_hemisphere(wo: Vec3, wi: Vec3) -> bool {
    wo.z * wi.z > 0.0
}

pub fn quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let det = b * b - 4.0 * a * c;
    if det < 0.0 {
        None
    } else {
        let t1 = (-b + det.sqrt()) / (2.0 * a);
        let t2 = (-b - det.sqrt()) / (2.0 * a);
        Some((t1, t2))
    }
}
pub fn transform_interaction(transform: Mat4, inter: &mut SurfaceInteraction<'_>) {
    let common = transform_common(transform, inter.common);
    let shading = transform_shading(transform, inter.shading);
    inter.common = common;
    inter.shading = shading;
}
fn transform_common(transform: Mat4, common: InteractionCommon) -> InteractionCommon {
    let p = transform.transform_point3(common.p);
    let n = transform
        .inverse()
        .transpose()
        .transform_vector3(common.normal)
        .normalize();
    let wo = transform.transform_vector3(common.w0).normalize();
    InteractionCommon {
        w0: wo,
        p,
        normal: n,
        time: common.time,
        uv: common.uv,
    }
}
fn transform_shading(transform: Mat4, shading: Shading) -> Shading {
    let dpdu = transform.transform_vector3(shading.dpdu);
    let dpdv = transform.transform_vector3(shading.dpdv);
    let dndu = transform.transform_vector3(shading.dndu);
    let dndv = transform.transform_vector3(shading.dndv);
    Shading::new(dpdu, dpdv, dndu, dndv)
}
pub fn compute_d2(dpdu: Vec3, dpdv: Vec3, d2pduu: Vec3, d2pduv: Vec3, d2pdvv: Vec3) -> (Vec3, Vec3, Vec3) {
    let e = dpdu.dot(dpdu);
    let f = dpdu.dot(dpdv);
    let g = dpdv.dot(dpdv);
    let n = dpdu.cross(dpdv).normalize();
    let ee = n.dot(d2pduu);
    let ff = n.dot(d2pduv);
    let gg = n.dot(d2pdvv);
    let inv_egf = 1.0 / (e * g - f * f);
    let dndu = (ff * f - ee * g) * inv_egf * dpdu + (ee * f - ff * e) * inv_egf * dpdv;
    let dndv = (gg * f - ff * g) * inv_egf * dpdu + (ff * f - gg * e) * inv_egf * dpdv;
    (n, dndu, dndv)
}