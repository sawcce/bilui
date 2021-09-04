use super::element::Element;
use super::types::{Children, V2i32, Position};
use super::data::{Computed};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum Direction {
    Row,
    Column,
}

pub struct Flex {
    children: Children,
    gap: u32,
    direction: Direction,
    computed: Computed,
}

impl Flex {
    pub fn new(children: Children, gap: u32, direction: Direction) -> Flex {
        Flex { children, gap, direction: direction, computed: Computed::new()}
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
        let parent_offset = match parent_offset {
            None => V2i32(0,0),
            Some(x) => x,
        };

        let mut offset = 0;
        let children = &self.children;

        for child in &mut self.children {
            let computed_offset = match self.direction {
                Direction::Row => V2i32(parent_offset.0 + offset, parent_offset.1),
                Direction::Column => V2i32(parent_offset.0, parent_offset.1 + offset)
            };

            child.render((size.0, size.1), canvas, Some(computed_offset));

            let c = &child;
            match self.direction {
                Direction::Row => offset += c.computed().size.x as i32,
                Direction::Column => offset += c.computed().size.y as i32
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
