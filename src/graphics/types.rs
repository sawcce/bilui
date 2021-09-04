use super::element::Element;

pub struct Position(pub u32, pub u32);
pub struct V2i32(pub i32, pub i32);
pub type Children = Vec<Box<dyn Element>>;
