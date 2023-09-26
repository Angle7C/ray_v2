use super::interaction::InteractionCommon;

pub struct VisibilityTester{
    a:InteractionCommon,
    b:InteractionCommon    
}
impl VisibilityTester{
    pub fn new(a:InteractionCommon,b:InteractionCommon)->Self{
        Self { a, b }
    }
}