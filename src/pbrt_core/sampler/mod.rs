use std::time::SystemTime;

use glam::{f64::DVec2,f32::Vec2};
use rand::{rngs::StdRng, SeedableRng, Rng};

pub struct Sampler {
    rand: StdRng,
   pub  num:usize
}
impl Default for Sampler {
    fn default() -> Self {
        unsafe {
            Sampler {
                rand: SeedableRng::seed_from_u64(
                    SystemTime::now().elapsed().unwrap_unchecked().as_secs(),
                ),
                num:1
            }
        }
    }
}
impl Clone for Sampler{
    fn clone(&self) -> Self {
        unsafe {
            Sampler {
                rand: SeedableRng::seed_from_u64(
                    SystemTime::now().elapsed().unwrap_unchecked().as_secs(),
                ),
                num:self.num
            }
        }
    }
}
impl Sampler {
    pub fn new(num:usize)->Self{
        unsafe {
            Sampler {
                rand: SeedableRng::seed_from_u64(
                    SystemTime::now().elapsed().unwrap_unchecked().as_secs(),
                ),
                num
            }
        }
    }
    pub fn sample_1d_d(&mut self)->f64{
        self.rand.gen_range(0.0..1.0)
    }
    pub fn sample_2d_d(&mut self)->DVec2{
        let x=self.sample_1d_d();
        let y=self.sample_1d_d();
        DVec2{x,y }
    }
    pub fn sample_2d(&mut self)->Vec2{
        self.sample_2d_d().as_vec2()
    }
    pub fn sample_d(&mut self)->f32{
        self.sample_1d_d() as f32
    }
}
