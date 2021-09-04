use sdl2::render::Canvas;
use sdl2::video::Window;

use super::types::{V2i32, Position};
use super::data::{Computed};

pub trait Element {
    fn render(
        &mut self,
        size: (u32, u32),
        canvas: &mut Canvas<Window>,
        parent_offset: Option<V2i32>,
    );

    fn computed(&self) -> &Computed;
    fn set_margin(&mut self, margin: Position);
}
