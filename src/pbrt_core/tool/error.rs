use std::fmt::Display;
#[derive(Debug)]
pub struct BizError{
    message:String,
}

impl Display for BizError{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}