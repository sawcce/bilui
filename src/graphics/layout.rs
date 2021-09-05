use super::data::{Computed, Size};
use super::element::Element;
use super::types::{Children, Position, V2i32};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum Direction {
    Row,
    Column,
}

pub enum ChildSize {
    Fractional,
    Absolute,
}

pub struct Flex {
    children: Children,
    gap: u32,
    size: Size,
    direction: Direction,
    children_size: ChildSize,
    computed: Computed,
}

impl Flex {
    pub fn new(
        children: Children,
        size: Size,
        gap: u32,
        direction: Direction,
        children_size: ChildSize,
    ) -> Flex {
        Flex {
            children,
            size,
            gap,
            direction: direction,
            computed: Computed::new(),
            children_size,
        }
    }

    fn set_gap(&mut self, gap: u32) {
        self.gap = gap;
    }
}

impl Element for Flex {
    fn render(
        &mut self,
        size: (u32, u32),
        canvas: &mut Canvas<Window>,
        parent_offset: Option<V2i32>,
    ) {
        let mut width = ((self.size.x as f32 / 100f32) * size.0 as f32) as u32;
        let mut height = ((self.size.y as f32 / 100f32) * size.1 as f32) as u32;

        let mut child_size = (width, height);

        match self.children_size {
            ChildSize::Fractional => {
                match self.direction {
                    Direction::Row => child_size.0 = width / self.children.len() as u32,
                    Direction::Column => child_size.1 = height / self.children.len() as u32,
                };
            },
            _ => {}
        }

        let parent_offset = match parent_offset {
            None => V2i32(0, 0),
            Some(x) => x,
        };

        self.computed.size = Size::new(width, height);

        let mut offset = 0;
        let children = &self.children;

        for child in &mut self.children {
            let computed_offset = match self.direction {
                Direction::Row => V2i32(parent_offset.0 + offset, parent_offset.1),
                Direction::Column => V2i32(parent_offset.0, parent_offset.1 + offset),
            };

            child.render(child_size, canvas, Some(computed_offset));

            let c = &child;
            match self.direction {
                Direction::Row => offset += c.computed().size.x as i32,
                Direction::Column => offset += c.computed().size.y as i32,
            };
        }
    }

    fn set_margin(&mut self, margin: Position) {
        self.gap = match self.direction {
            Direction::Row => (margin.0 as u32),
            Direction::Column => (margin.1 as u32),
            _ => 0u32,
        }
    }

    fn computed(&self) -> &Computed {
        return &self.computed;
    }
}
