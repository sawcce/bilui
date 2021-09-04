use sdl2::render::Canvas;
use sdl2::video::Window;

use super::types::{V2i32, Position};

pub trait Element {
    fn render(
        &mut self,
        size: (u32, u32),
        canvas: &mut Canvas<Window>,
        parent_offset: Option<V2i32>,
    );
    fn set_margin(&mut self, margin: Position);
    fn set_offset(&mut self, offset: V2i32);
}
