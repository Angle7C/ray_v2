use super::{interaction::InteractionCommon, sence::Sence};
#[derive(Default)]
pub struct VisibilityTester{
    a:InteractionCommon,
    b:InteractionCommon    
}
impl VisibilityTester{
    pub fn new(a:InteractionCommon,b:InteractionCommon)->Self{
        Self { a, b }
    }
    pub fn unoccluded(&self,sence:&Sence)->bool{
        unimplemented!()
    }
}