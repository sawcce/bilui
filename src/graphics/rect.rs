use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug)]

pub struct Position(pub u32, pub u32);
pub struct V2i32(i32, i32);
pub struct Children(pub Vec<Box<dyn Element>>);

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

pub struct ScaledRect {
    width: f32,
    height: f32,
    pixel_offset: V2i32,
    margin: Position,
    color: Color,
    children: Vec<Box<dyn Element>>,
}

impl ScaledRect {
    pub fn new(
        width: f32,
        height: f32,
        children: Vec<Box<dyn Element>>,
        color: Color,
    ) -> ScaledRect {
        return ScaledRect {
            width,
            height,
            color,
            pixel_offset: V2i32(0, 0),
            margin: Position(0, 0),
            children,
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

        let mut width = ((self.width / 100f32) * size.0 as f32) as u32;
        let mut height = ((self.height / 100f32) * size.1 as f32) as u32;

        width -= self.margin.0 * 2;
        height -= self.margin.1 * 2;

        println!("{}x{}, {:?}", (self.width / 100f32), height, size);

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

    fn set_offset(&mut self, offset: V2i32) {
        self.pixel_offset = offset;
    }
}
