use std::{sync::mpsc::{self, Sender, Receiver},   path::Path, time::Instant, ops::Sub, thread};

use glam::UVec2;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};
use log::info;


use crate::pbrt_core::tool::tile::merage_tile;

use self::{path::PathIntegrator, direct::DirectIntegrator};

use super::{tool::{sence::Sence, film::Film, RayDiff, tile::Tile, color::Color}, camera::{Camera, CameraSample}, sampler::Sampler};

pub mod path;
pub mod direct;
pub enum Integrator{
    Path(Box<PathIntegrator>),
    Direct(Box<DirectIntegrator>)
}
pub trait IntegratorAble{
    fn is_next(&self, dept: &mut usize) -> Option<f32>;
    fn fi(&self,ray:RayDiff,sence:&Sence,sampler:&mut Sampler)->Color;
}
impl IntegratorAble for Integrator{
    fn is_next(&self, dept: &mut usize) -> Option<f32> {
        match &self{
            Integrator::Path(path) => path.is_next(dept),
            _=>None
        }
    }

    fn fi(&self,ray:RayDiff,sence:&Sence,sampler:&mut Sampler)->Color {
        match  &self {
            Integrator::Path(path) => path.fi(ray, sence, sampler),
            Integrator::Direct(direct)=>direct.fi(ray, sence, sampler)
        }
    }
}
impl Integrator{
    pub fn render_process(self, name: &str, core_num: u64, sence: &Sence,size:UVec2,sampler:Sampler) {
        let (sender, receiver) = mpsc::channel::<Vec<Tile>>();
        let film = Film::new(size);
        let camera = sence.camera;
        let t1=Instant::now();
        let (m,style) = pbr();
        let len=size.x/(core_num*2) as u32;

        thread::scope(|scope| {
            for _ in 0..core_num {
                let pb=m.add(ProgressBar::new(len as u64));
                pb.set_style(style.clone());
                scope.spawn(self.render_core(&film, &camera, sender.clone(), sence, sampler.clone(),pb));
            }
            drop(sender);
        });
        let t2=Instant::now();

        info!("渲染耗时:{} s",t2.sub(t1).as_secs_f32());
        Self::output(receiver, size, name, sampler.num);
        m.clear().unwrap();
    }
    fn render_core<'a, 'b>(
        &'b self,
        film: &'a Film,
        camera: &'a Camera,
        send: Sender<Vec<Tile>>,
        sence: &'a Sence,
        mut sampler:Sampler,
        pb:ProgressBar,
    ) -> impl FnOnce() + 'a where 'b: 'a,
    {
        move || {
            let n = sampler.num;
            // let mut sampler = sampler.clone();
            let mut tiles:Vec<Tile>=vec![];
            while let Some(item) = film.iter() {
                let index=item.index;
                let mut tile=Tile::new(index);
                for (u, v) in item {
                    let mut color = Color::ZERO;
                    for _ in 0..n {
                        let camera_sample = CameraSample::new(u, v, &mut sampler);
                        let ray = camera.generate_ray(camera_sample);
                        color += self.fi(ray, sence,&mut  sampler);
                    }
                    tile.push(color);
                }
                pb.inc(1);
                info!("index:{} 渲染完成",index);
                tiles.push(tile);
            }
            pb.finish();
            send.send(tiles).expect("send 失败");
        }
    }
    fn output(rece: Receiver<Vec<Tile>>, size: UVec2, name: &str, num: usize) {
        let mut list:Vec<Vec<Tile>>=vec![];
        for iter in rece.iter() {
            list.push(iter);
        }
        let buffer = merage_tile(list, size);
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", size.x, size.y));
        format!("渲染完成，图像输出:{}", path.display());
        buffer.write(image::ImageFormat::Jpeg, num as f32, path);
    }

    pub fn render_process_debug(self, name: &str, num: u64, sence: &Sence,size:UVec2) {
        let film = Film::new(size);
        let bar_size =size.x * size.y;
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
                    color += self.fi(ray, sence,&mut sampler);
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
pub fn to_color(color:Color,ssp:f32)->Rgb<u8>{
        let vec = (color / ssp).powf(2.0);
        let rgb = vec * 255.0;
        let color= Rgb([
            rgb.x.clamp(0.0, 255.0) as u8,
            rgb.y.clamp(0.0, 255.0) as u8,
            rgb.z.clamp(0.0, 255.0) as u8,
        ]);
        color
}

pub fn pbr()->(MultiProgress,ProgressStyle){
    let m=MultiProgress::new();
    let sty=ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"
    ).unwrap().progress_chars("##-");
    (m,sty)
    
}