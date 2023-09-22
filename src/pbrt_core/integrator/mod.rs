use std::{
    ops::Sub,
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Instant,
};

use glam::{UVec2, Vec2, Vec3};
use image::{Rgb, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::info;
use rand::seq::index;

use crate::pbrt_core::bxdf::BxDFType;
use crate::pbrt_core::light::{Light, LightAble, LightType};
use crate::pbrt_core::tool::{InteractionCommon, SurfaceInteraction, Visibility};
use crate::pbrt_core::{sampler, tool::tile::merage_tile};

use self::{direct::DirectIntegrator, path::PathIntegrator};

use super::{
    camera::{Camera, CameraSample},
    sampler::Sampler,
    tool::{color::Color, film::Film, sence::Sence, tile::Tile, RayDiff},
};

pub mod direct;
pub mod path;

pub enum Integrator {
    Path(Box<PathIntegrator>, usize, Sampler),
    Direct(Box<DirectIntegrator>, usize, Sampler),
}

pub trait IntegratorAble {
    fn is_next(&self, dept: &mut usize) -> Option<f32>;
    fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> Color;
}

impl IntegratorAble for Integrator {
    fn is_next(&self, dept: &mut usize) -> Option<f32> {
        match &self {
            Integrator::Path(path, _, _) => path.is_next(dept),
            _ => None,
        }
    }

    fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> Color {
        match &self {
            Integrator::Path(path, _, _) => path.fi(ray, sence, sampler),
            Integrator::Direct(direct, _, _) => direct.fi(ray, sence, sampler),
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
    pub fn render_process(self, name: &str, sence: &Sence, size: UVec2) {
        let (sender, receiver) = mpsc::channel::<Vec<Tile>>();
        let film = Film::new(size);
        let camera = sence.camera;
        let t1 = Instant::now();
        let (m, style) = pbr();
        let core = self.get_num();
        let len = size.x / (core * 2) as u32;
        let num = self.get_sample().num;
        thread::scope(|scope| {
            for _ in 0..core {
                let pb = m.add(ProgressBar::new(len as u64));
                pb.set_style(style.clone());
                scope.spawn(self.render_core(
                    &film,
                    &camera,
                    sender.clone(),
                    sence,
                    self.get_sample(),
                    pb,
                ));
            }
            drop(sender);
        });
        let t2 = Instant::now();

        info!("渲染耗时:{} s", t2.sub(t1).as_secs_f32());
        Self::output(receiver, size, name, num);
        m.clear().unwrap();
    }
    fn render_core<'a, 'b>(
        &'b self,
        film: &'a Film,
        camera: &'a Camera,
        send: Sender<Vec<Tile>>,
        sence: &'a Sence,
        mut sampler: Sampler,
        pb: ProgressBar,
    ) -> impl FnOnce() + 'a
    where
        'b: 'a,
    {
        move || {
            let n = sampler.num;
            // let mut sampler = sampler.clone();
            let mut tiles: Vec<Tile> = vec![];
            while let Some(item) = film.iter() {
                let index = item.index;
                let mut tile = Tile::new(index);
                for (u, v) in item {
                    let mut color = Color::ZERO;
                    for _ in 0..n {
                        let camera_sample = CameraSample::new(u, v, &mut sampler);
                        let ray = camera.generate_ray(camera_sample);
                        color += self.fi(ray, sence, &mut sampler);
                    }
                    tile.push(color);
                }
                pb.inc(1);
                tiles.push(tile);
            }
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
        format!("渲染完成，图像输出:{}", path.display());
        println!("{}", path.display());
        buffer.write(image::ImageFormat::Jpeg, num as f32, path);
    }

    pub fn render_process_debug(self, name: &str, num: u64, sence: &Sence, size: UVec2) {
        let film = Film::new(size);
        let bar_size = size.x * size.y;
        let n = 1;
        let mut sampler = Sampler::default();
        let camera = sence.camera;
        let bar = ProgressBar::new(bar_size as u64);
        let mut image = RgbImage::new(size.x, size.y);
        while let Some(item) = film.iter() {
            for (u, v) in item {
                let mut color = Color::ZERO;
                for _ in 0..n {
                    let camera_sample = CameraSample::new(u, v, &mut sampler);
                    let ray = camera.generate_ray(camera_sample);
                    color += self.fi(ray, sence, &mut sampler);
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
    let color = Rgb([
        rgb.x.clamp(0.0, 255.0) as u8,
        rgb.y.clamp(0.0, 255.0) as u8,
        rgb.z.clamp(0.0, 255.0) as u8,
    ]);
    color
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
    sence: &Sence,
    mut sampler: Sampler,
    n_light_sample: Vec<i32>,
    handle_media: bool,
) -> Color {
    let mut l = Color::ZERO;

    for (index, light) in sence.light.iter().enumerate() {
        let mut ld = Vec3::ZERO;
        if n_light_sample[index] == 1 {
            let u_light = sampler.sample_2d_d();
            let u_scattering = sampler.sample_2d_d();
            ld += estimate_direct(
                common,
                light,
                sampler.sample_2d_d(),
                sence,
                sampler.clone(),
                handle_media,
                true,
            );
        } else {
            for i in 0..n_light_sample[index] {
                ld += estimate_direct(
                    common,
                    light,
                    sampler.sample_2d_d(),
                    sence,
                    sampler.clone(),
                    handle_media,
                    false,
                );
            }
            ld /= n_light_sample[index] as f32
        }
        l += ld;
    }
    l
}

pub fn estimate_direct(
    inter: &SurfaceInteraction,
    light: &Light,
    u_light: Vec2,
    sence: &Sence,
    mut sampler: Sampler,
    handle_media: bool,
    specular: bool,
) -> Color {
    let bxdf_flags = if specular {
        BxDFType::All.into()
    } else {
        BxDFType::All as u32 & !BxDFType::Specular
    };
    let mut ld = Vec3::ZERO;
    let mut wi: Vec3=Default::default();
    let mut pdf: f32 = 0.0;
    let mut vis: Visibility=Default::default();
    let mut light_common = InteractionCommon::default();
    let mut li = light.sample_li(
        &inter.common,
        &mut light_common,
        u_light,
        &mut wi,
        &mut pdf,
        &mut vis,
    );
    //合理的pdf和采样出光线
    if pdf > 0.0 && !li.abs_diff_eq(Vec3::ZERO, f32::EPSILON) {
        //计算BSDF
       let f= if let Some(ref bsdf) = inter.bsdf {
            bsdf.f(&inter.common.w0, &wi, bxdf_flags) * wi.dot(inter.shading.n).abs()
        }else{
            Vec3::ZERO
        };
        //介质缩减
        if handle_media {
            todo!()
        } else if !vis.is_vis(sence) {
            li = Color::ZERO;
        }
        //计算光贡献
        if !li.abs_diff_eq(Vec3::ZERO, f32::EPSILON) {
            if LightType::is_delta(light.get_type()) {
                ld += f * li *vis.g(sence)/ pdf ;
            } else {
                let weight = power_heuristic(1.0, pdf, 1.0, 1.0);
                ld += f * li * weight*vis.g(sence) / pdf;
            }
        }
    }
    //BSDF重要性采样
    if !LightType::is_delta(light.get_type()) {
        let mut sampled_specular = false;
        let mut smapled_type = BxDFType::None;
        if let Some(ref bsdf) = inter.bsdf {
            let mut f = bsdf.sample_f(
                &inter.common.w0,
                &mut wi,
                sampler.sample_2d_d(),
                &mut pdf,
                bxdf_flags,
            );
            f *= wi.dot(inter.common.w0).abs();
            sampled_specular = smapled_type & BxDFType::Specular > 0;
            let weight;
            if (!sampled_specular) {
                let light_pdf = light.pdf_li(inter, &wi);
                if light_pdf.abs() < f32::EPSILON {
                    return ld;
                }
                weight = power_heuristic(1.0, 1.0, 1.0, light_pdf);
            }
        } else {
        }
    };
    ld
}

pub fn power_heuristic(nf: f32, f_pdf: f32, ng: f32, g_pdf: f32) -> f32 {
    let f = nf * f_pdf;
    let g = ng * g_pdf;
    (f * f) / (f * f + g * g)
}
