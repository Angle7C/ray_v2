use glam::{UVec2, Vec2, Vec3};
use image::{Rgb, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::info;

use rand::Rng;
use std::{
    ops::Sub,
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Instant,
};

use crate::pbrt_core::bxdf::BxDFType;
use crate::pbrt_core::light::{LightAble, LightType};
use crate::pbrt_core::tool::tile::merage_tile;
use crate::pbrt_core::tool::{InteractionCommon, SurfaceInteraction, Visibility};

use self::{direct::DirectIntegrator, path::PathIntegrator};

use super::{
    camera::{Camera, CameraSample},
    primitive::{shape::ShapeAble, Primitive},
    sampler::{self, Sampler},
    tool::{color::Color, film::Film, sence::Scene, tile::Tile, Ray, RayDiff},
};

pub mod direct;
pub mod path;

pub enum Integrator {
    Path(Box<PathIntegrator>, usize, Sampler),
    Direct(Box<DirectIntegrator>, usize, Sampler),
}

pub trait IntegratorAble {
    //求解 irradince
    fn fi(&self, ray: RayDiff, sence: &Scene, sampler: &mut Sampler,
          #[cfg(debug_assertions)]
        i:&mut i32
    ) -> Color;
}

impl IntegratorAble for Integrator {
    fn fi(&self, ray: RayDiff, sence: &Scene, sampler: &mut Sampler,
          #[cfg(debug_assertions)]
        i:&mut i32
    ) -> Color {
        match &self {
            Integrator::Path(path, _, _) => path.fi(ray, sence, sampler,
                #[cfg(debug_assertions)]
                i),
            Integrator::Direct(direct, _, _) => direct.fi(ray, sence, sampler,
                #[cfg(debug_assertions)]
                i),
        }
    }
}

impl Integrator {
    fn get_num(&self) -> usize {
        match &self {
            Integrator::Path(_, index, _) => *index,
            Integrator::Direct(_, index, _) => *index,
        }
    }
    fn get_sample(&self) -> Sampler {
        match &self {
            Integrator::Path(_, _, sampler) => sampler.clone(),
            Integrator::Direct(_, _, sampler) => sampler.clone(),
        }
    }
    pub fn render_process(self, name: &str, scene: &Scene, size: UVec2) {
        let (sender, receiver) = mpsc::channel::<Vec<Tile>>();
        let film = Film::new(size);
        let camera = scene.camera;
        let t1 = Instant::now();
        let (m, style) = pbr();
        let core = self.get_num();
        let len = film.render_size() / (core*4) as u32;
        let num = self.get_sample().num;
        thread::scope(|scope| {
            for i in 0..core {
                let pb = m.add(ProgressBar::new(len as u64));
                pb.set_style(style.clone());
                scope.spawn(self.render_core(
                    &film,
                    &camera,
                    sender.clone(),
                    scene,
                    self.get_sample(),
                    pb,
                    i
                ));
            }
            drop(sender);
        });
        let t2 = Instant::now();

        info!("渲染耗时:{} s", t2.sub(t1).as_secs_f32());
        Self::output(receiver, size, name, num);
        m.clear().unwrap();
    }
    //核心渲染函数
    fn render_core<'a, 'b>(
        &'b self,
        film: &'a Film,
        camera: &'a Camera,
        send: Sender<Vec<Tile>>,
        scene: &'a Scene,
        mut sampler: Sampler,
        pb: ProgressBar,
        index:usize
    ) -> impl FnOnce() + 'a
    where
        'b: 'a,
    {
        move || {
            let n = sampler.num;
            let mut i=0;
            let mut tiles: Vec<Tile> = vec![];
            while let Some(item) = film.iter() {
                let index = item.index;
                let mut tile = Tile::new(index);
                for (u, v) in item {
                    let mut color = Color::ZERO;
                    i=0;
                    //ssp 抗锯齿
                    for _ in 0..n {
                        //相机采样
                        let camera_sample = CameraSample::new(u, v, &mut sampler);
                        //光线采样
                        let ray = camera.generate_ray(camera_sample);
                        //颜色生成
                        color += self.fi(ray, scene, &mut sampler,
                            #[cfg(debug_assertions)]
                            &mut i
                        );
                    }
                    tile.push(color);
                }
                
                pb.inc(1);
                tiles.push(tile);
            }
            info!("thread {} close",index);
            pb.finish();
            send.send(tiles).expect("send 失败");
        }
    }
    fn output(rece: Receiver<Vec<Tile>>, size: UVec2, name: &str, num: usize) {
        let mut list: Vec<Vec<Tile>> = vec![];
        for iter in rece.iter() {
            list.push(iter);
        }
        let buffer = merage_tile(list, size);
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", size.x, size.y));
        
        buffer.write(image::ImageFormat::Jpeg, num as f32, &path);
        println!("{}", format!("渲染完成，图像输出:{}", path.display()));

    }

    pub fn render_process_debug(self, name: &str, num: u64, sence: &Scene, size: UVec2) {
        let film = Film::new(size);
        let bar_size = size.x * size.y;
        let n = 1;
        let mut sampler = Sampler::default();
        let camera = sence.camera;
        let bar = ProgressBar::new(bar_size as u64);
        let mut image = RgbImage::new(size.x, size.y);
        let mut i=0;
        while let Some(item) = film.iter() {
            for (u, v) in item {
                let mut color = Color::ZERO;
                for _ in 0..n {
                    let camera_sample = CameraSample::new(u, v, &mut sampler);
                    let ray = camera.generate_ray(camera_sample);
                    color += self.fi(ray, sence, &mut sampler,
                        #[cfg(debug_assertions)]
                        &mut i
                    );
                }
                image.put_pixel(u as u32, v as u32, to_color(color, num as f32));
                bar.inc(1);
            }
        }
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", size.x, size.y));
        format!("渲染完成，图像输出:{}", path.display());
        bar.finish_with_message("渲染完成，图像输出");
        image
            .save_with_format(path, image::ImageFormat::Jpeg)
            .expect("图片保存失败");
    }
}

pub fn to_color(color: Color, ssp: f32) -> Rgb<u8> {
    let vec = (color / ssp).powf(2.0);
    let rgb = vec * 255.0;
    
    Rgb([
        rgb.x.clamp(0.0, 255.0) as u8,
        rgb.y.clamp(0.0, 255.0) as u8,
        rgb.z.clamp(0.0, 255.0) as u8,
    ])
}

pub fn pbr() -> (MultiProgress, ProgressStyle) {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    (m, sty)
}

pub fn uniform_sample_all_light(
    common: &SurfaceInteraction,
    sence: &Scene,
    sampler: Sampler,
    handle_media: bool,
) -> Color {
    let len = sence.lights.len();
    if len==0{
        return Color::ZERO;
    }
    let mut ld = Color::default();
    let mut sampler=sampler.clone();
    let mut light_shape_index=0;
    for light in &sence.lights {
        let smaple = light.get_samples();
        let mut li=Color::ZERO;
        let u = sampler.sample_2d();
        let shape = sence.shapes_light.get(light_shape_index).and_then(|shape|Some(shape.as_ref()));
        for _ in 0..smaple {
            li += estimate_direct(
                common,
                light.as_ref(),
                shape,
                u,
                sence,
                sampler.clone(),
                handle_media,
                false,
            );
        }
        light_shape_index+=1;
        ld+=li / smaple as f32;
    }
    ld
}

pub fn unifrom_sample_one_light(
    common: &SurfaceInteraction,
    sence: &Scene,
    mut sampler: Sampler,
    handle_media: bool,
) -> Color {
    let len = sence.lights.len();
    let num = sampler.rand.gen_range(0..len);
    let light = sence.lights[num].as_ref();
    let mut ld = Color::default();
    let smaple = light.get_samples();
    let shape = sence.shapes_light.get(num).and_then(|shape|Some(shape.as_ref()));
    for _ in 0..smaple {
        ld += estimate_direct(
            common,
            light,
            shape,
            sampler.sample_2d(),
            sence,
            sampler.clone(),
            handle_media,
            false,
        );
    }

    ld * len as f32 / smaple  as f32
  
}

pub fn estimate_direct(
    inter: &SurfaceInteraction,
    light: &dyn LightAble,
    shape: Option<&dyn ShapeAble>,
    u_light: Vec2,
    sence: &Scene,
    mut sampler: Sampler,
    _handle_media: bool,
    specular: bool,
) -> Color {
    let bxdf_flags = if specular {
        BxDFType::All.into()
    } else {
        BxDFType::All as u32 & !BxDFType::Specular
    };
    let mut ld = Color::ZERO;
    let mut wi: Vec3 = Vec3::ZERO;
    let mut light_pdf: f32 = 0.0;
    let mut vis: Visibility = Default::default();
    let mut light_common: InteractionCommon = Default::default();
    let mut li = light.sample_li(
        &inter.common,
        &mut light_common,
        shape,
        u_light,
        &mut wi,
        &mut light_pdf,
        &mut vis,
    );
    // return li;
    // 合理的pdf和采样出光线 
    if light_pdf > 0.0 && !li.abs_diff_eq(0.0, f32::EPSILON) {
        //计算BSDF
        let f = if let Some(ref bsdf) = inter.bsdf {
            bsdf.f(&inter.common.w0, &wi, bxdf_flags) * wi.dot(inter.shading.n).abs()
        } else {
            Vec3::ZERO
        };
        //计算光贡献
        if !f.abs_diff_eq(Vec3::ZERO, f32::EPSILON) && !vis.is_vis(sence) {
            li = Color::ZERO;
        }
        let scattle_pdf = if let Some(ref bsdf) = inter.bsdf {
            bsdf.pdf(&inter.common.w0, &-wi, bxdf_flags)
        } else {
            1.0
        };

        if !li.abs_diff_eq(0.0, f32::EPSILON) {
            if LightType::is_delta(light.get_type()) {
                ld +=  li *f* vis.g(sence) / light_pdf;
            }else if LightType::is_inf(light.get_type()){
                ld+=li*f*vis.g_inf(sence) / light_pdf;
            }
             else {
                let weight = power_heuristic(1.0, light_pdf, 1.0, scattle_pdf);
                ld +=  li *f * vis.g(sence)*weight/light_pdf;
            }
        }
    }
    //BSDF重要性采样
    if !LightType::is_delta(light.get_type()) {
        let mut sampled_specular = false;
        let mut smapled_type = BxDFType::None as u32;
        let mut bsdf_pdf = 0.0;
        if let Some(ref bsdf) = inter.bsdf {
            let f = bsdf.sample_f(
                &inter.common.w0,
                &mut wi,
                sampler.sample_2d_d(),
                &mut bsdf_pdf,
                bxdf_flags,
                &mut smapled_type,
            ) * wi.dot(inter.shading.n).abs();
            sampled_specular = BxDFType::Specular as u32 & smapled_type > 0;
            if !f.abs_diff_eq(Vec3::ZERO, f32::EPSILON) && bsdf_pdf > 0.0 {
                let weight = if !sampled_specular {
                    let light_pdf = light.pdf_li(&inter.common, &wi,shape);
                    if light_pdf.abs() < f32::EPSILON {
                        return ld;
                    }
                    power_heuristic(1.0, bsdf_pdf, 1.0, light_pdf)
                } else {
                    1.0
                };
                let ray = RayDiff::new(Ray::new(inter.common.p, -wi));
                let li =
                if let Some(ref light_inter) = sence.intersect(ray) {
                    light_inter.le(ray)
                }else{
                    Color::ZERO
                };
                if !li.abs_diff_eq(0.0, f32::EPSILON) {
                    ld += li * (f * weight / bsdf_pdf);
                }
            }
        }
    };
    ld
}

pub fn power_heuristic(nf: f32, f_pdf: f32, ng: f32, g_pdf: f32) -> f32 {
    let f = nf * f_pdf;
    let g = ng * g_pdf;
    (f * f) / (f * f + g * g)
}

// pub fn get_light(
//     inter: &SurfaceInteraction,
//     sence: &Scene,
//     mut sampler: Sampler,
// ) -> Color {
//     if sence.light.is_empty() {
//         return Color::ZERO;
//     }
//     let num: usize = sampler.rand.gen_range(0..sence.light.len());
//     let light = &sence.light[num];
//     let mut light_common = Default::default();
//     let mut wi = Vec3::default();
//     let mut pdf = 0.0;
//     let mut vis = Visibility::default();
//     let li = light.sample_li(
//         &inter.common,
//         &mut light_common,
//         sampler.sample_2d(),
//         &mut wi,
//         &mut pdf,
//         &mut vis,
//     );
//     let f = if let Some(ref bsdf) = inter.bsdf {
//         bsdf.f(&inter.common.w0, &wi, BxDFType::All.into())
//          * wi.dot(inter.shading.n).abs()
//     } else {
//         Vec3::ZERO
//     };
//     // return  f;
//     vis.g(sence)* f *li/pdf
// }
