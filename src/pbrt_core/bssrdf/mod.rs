use super::tool::SurfaceInteraction;

pub struct BSSRDF<'a>{
    po:&'a SurfaceInteraction<'a>,
    eta: f32,
}