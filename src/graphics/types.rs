use super::element::Element;

#[derive(Copy, Clone)]
pub struct Position(pub u32, pub u32);

#[derive(Copy, Clone)]
pub struct V2i32(pub i32, pub i32);

pub type Children =  Vec<Box<dyn Element>>;
