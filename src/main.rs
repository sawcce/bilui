extern crate sdl2;

mod graphics;

use graphics::element::Element;
use graphics::types::Position;
use graphics::rect::ScaledRect;
use graphics::layout::{Direction, ChildSize, Flex};
use graphics::data::{Size};
use graphics::types::{Children};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Bilui", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(12, 12, 12));
    canvas.clear();
    canvas.present();

    let column = Direction::Column;
    let fr = ChildSize::Fractional;
    let surface = Color::RGBA(50, 50, 50, 255);

    // Create rectangle that has 100% of the width of the parent and take half the height of the parent
    let mut rect1 = ScaledRect::new(Size::new(100, 100), vec![], surface);
    rect1.set_margin(Position(10,10));
    let mut rect2 = ScaledRect::new(Size::new(100, 100), vec![], surface);
    rect2.set_margin(Position(10,10));

    let mut rect3 = ScaledRect::new(Size::new(100, 100), vec![], surface);
    rect3.set_margin(Position(10,10));

    // Create a column that contains the two boxes
    let children: Children = vec![Box::new(rect1), Box::new(rect2)];
    let mut col1 = Flex::new(children, Size::new(100, 100), 0, column, fr);

    // Create a row to contain the two columns (making a grid);
    let children: Children = vec![Box::new(col1), Box::new(rect3)];
    let mut main_row = Flex::new(children, Size::new(100, 100),  0, Direction::Row, ChildSize::Fractional);

    let mut i = 1;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(12, 12, 12));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        // A draw a rectangle which almost fills our window with it !

        let window = canvas.window_mut();
        let size = window.size();

        main_row.render(size, &mut canvas, None);
        //col1.set_margin(Position(i, i));

        i += 1;
        i = i % 1000;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

