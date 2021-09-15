use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::data::{Computed, Size};
use super::element::Element;
use super::types::{Children, Position, V2i32};

pub struct ScaledRect {
    size: Size,
    margin: Position,
    color: Color,
    children: Children,
    computed: Computed,
}


impl ScaledRect {
    pub fn new(size: Size, children: Children, color: Color) -> ScaledRect {
        return ScaledRect {
            size,
            color,
            margin: Position(0, 0),
            children,
            computed: Computed::new(),
        };
    }
}

impl Element for ScaledRect {
    fn render(
        &mut self,
        size: (u32, u32),
        canvas: &mut Canvas<Window>,
        parent_offset: Option<V2i32>,
    ) {
        let mut offset = V2i32(self.margin.0 as i32, self.margin.1 as i32);

        match parent_offset {
            None => {}
            Some(x) => {
                offset.0 += x.0;
                offset.1 += x.1;
            }
        }

        let mut width = ((self.size.x as f32/ 100f32) * size.0 as f32) as u32;
        let mut height = ((self.size.y as f32/ 100f32) * size.1 as f32) as u32;
        self.computed.size = Size::new(width, height);

        width -= self.margin.0 * 2;
        height -= self.margin.1 * 2;

        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(Rect::new(offset.0, offset.1, width, height));

        let children = &self.children;

        for child in &mut self.children {
            child.render((width, height), canvas, Some(V2i32(offset.0, offset.1)));
        }
    }

    fn set_margin(&mut self, margin: Position) {
        self.margin = margin;
    }

    fn computed(&self) -> &Computed {
        return &self.computed;
    }
}
