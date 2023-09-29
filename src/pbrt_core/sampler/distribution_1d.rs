use std::cmp::Ordering;

pub struct Distribution1D {
    func: Vec<f32>,
    cdf: Vec<f32>,
    func_int: f32,
}
impl Distribution1D {
    pub fn new(func: &[f32], n: usize) -> Self {
        let func: Vec<f32> = func.iter().map(|i| *i).collect::<Vec<_>>();
        let mut cdf = vec![];
        cdf.push(0.0);
        for i in 1..n + 1 {
            cdf.push(cdf[i - 1] + func[i - 1] / n as f32);
        }
        let func_int = cdf[n];
        if func_int == 0.0 {
            for i in 1..n + 1 {
                cdf[i] = i as f32 / n as f32;
            }
        } else {
            for i in 1..n + 1 {
                cdf[i] /= func_int;
            }
        };
        Self {
            func,
            cdf,
            func_int,
        }
    }
    pub fn count(&self) -> usize {
        self.func.len()
    }
    //二分查找
    pub fn sample_continuous(&self, u: f32, _pdf: &[f32], off: Option<&mut i32>) -> f32 {
        let ans = self.cdf.binary_search_by(|x| {
            if *x <= u {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let index = match ans {
            Ok(index) => index,
            Err(index) => index,
        };
        if let Some( off)=off{
            *off=self.cdf[index] as i32;
        };
        let mut  du=u-self.cdf[index];
        if self.cdf[index+1]-self.cdf[index]>0.0{
            du/=self.cdf[index+1]-self.cdf[index];
        };
        unimplemented!()
    }
}
