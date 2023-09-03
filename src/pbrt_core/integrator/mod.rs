use std::{sync::mpsc::{self, Sender, Receiver}, thread::{self, Scope},  path::Path, time::Instant, ops::Sub};

use glam::{f64::DVec3, UVec2};
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use log::info;


use crate::pbrt_core::tool::tile::merage_tile;

use self::path::PathIntegrator;

use super::{tool::{sence::Sence, film::{self, Film}, RayDiff, tile::Tile, color::Color}, camera::{Camera, CameraSample}, sampler::Sampler};

pub mod path;
pub mod direct;
pub enum Integrator{
    Path(Box<PathIntegrator>)
}
pub trait IntegratorAble{
    fn is_next(&self, dept: &mut usize) -> Option<f64>;
    fn fi(&self,ray:RayDiff,sence:&Sence,sampler:&mut Sampler)->DVec3;
}
impl IntegratorAble for Integrator{
    fn is_next(&self, dept: &mut usize) -> Option<f64> {
        match &self{
            Integrator::Path(path) => path.is_next(dept)
        }
    }

    fn fi(&self,ray:RayDiff,sence:&Sence,sampler:&mut Sampler)->Color {
        match  &self {
            Integrator::Path(path) => path.fi(ray, sence, sampler),
        }
    }
}
impl Integrator{
    pub fn render_process(self, name: &str, core_num: u64, sence: &Sence,size:UVec2,sampler:Sampler) {
        let (sender, receiver) = mpsc::channel::<Vec<Tile>>();
        let film = Film::new(size);
        let camera = sence.camera;
        let t1=Instant::now();
        thread::scope(|scope| {
            for _ in 0..core_num {
                scope.spawn(self.render_core(&film, &camera, sender.clone(), sence, sampler.clone()));
            }
            drop(sender);
        });
        let t2=Instant::now();
        info!("渲染耗时:{} s",t2.sub(t1).as_secs_f32());
        Self::output(receiver, size, name, sampler.num)
    }
    fn render_core<'a, 'b>(
        &'b self,
        film: &'a Film,
        camera: &'a Camera,
        send: Sender<Vec<Tile>>,
        sence: &'a Sence,
        mut sampler:Sampler,
    ) -> impl FnOnce() + 'a where 'b: 'a,
    {
        move || {
            let n = sampler.num;
            let mut sampler = sampler.clone();
            let mut tiles:Vec<Tile>=vec![];
            while let Some(item) = film.iter() {
                let index=item.index;
                let mut tile=Tile::new(index);
                for (u, v) in item {
                    let mut color = DVec3::ZERO;
                    for _ in 0..n {
                        let camera_sample = CameraSample::new(u, v, &mut sampler);
                        let ray = camera.generate_ray(camera_sample);
                        color += self.fi(ray, sence,&mut  sampler);
                    }
                    tile.push(color);
                }
                info!("index:{} 渲染完成",index);
                tiles.push(tile);
            }
            send.send(tiles).expect("send 失败");
        }
    }
    fn output(rece: Receiver<Vec<Tile>>, size: UVec2, name: &str, num: usize) {
        let bar_size = size.x/2;
        let mut bar = ProgressBar::new(bar_size as u64);
        let mut image = RgbImage::new(size.x, size.y);
        let mut list:Vec<Vec<Tile>>=vec![];
        for iter in rece.iter() {
            list.push(iter);
        }
        let buffer = merage_tile(list, size);
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", size.x, size.y));
        format!("渲染完成，图像输出:{}", path.display());
        buffer.write(image::ImageFormat::Jpeg, num as f64, path);
    }

    pub fn render_process_debug(self, name: &str, num: u64, sence: &Sence,size:UVec2,sampler:Sampler) {
        let (sender, receiver) = mpsc::channel::<(u64, u64, DVec3)>();
        let film = Film::new(size);
        let bar_size =size.x * size.y;
        let n = sampler.num;
        let mut sampler = sampler.clone();
        let camera = sence.camera;
        let mut bar = ProgressBar::new(bar_size as u64);
        let mut image = RgbImage::new(size.x, size.y);
        while let Some(item) = film.iter() {
            for (u, v) in item {
                let mut color = DVec3::ZERO;
                for _ in 0..n {
                    let camera_sample = CameraSample::new(u, v, &mut sampler);
                    let ray = camera.generate_ray(camera_sample);
                    color += self.fi(ray, sence,&mut sampler);
                }
                image.put_pixel(u as u32, v as u32, to_color(color, num as f64));
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
pub fn to_color(color:DVec3,ssp:f64)->Rgb<u8>{
        let vec = (color / ssp);
        let rgb = vec * 255.0;
        let color= Rgb([
            rgb.x.clamp(0.0, 255.0) as u8,
            rgb.y.clamp(0.0, 255.0) as u8,
            rgb.z.clamp(0.0, 255.0) as u8,
        ]);
        color
}