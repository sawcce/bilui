extern crate sdl2;

mod graphics;

use graphics::element::Element;
use graphics::types::Position;
use graphics::rect::ScaledRect;
use graphics::layout::{Direction, Flex};
use graphics::data::{Size};

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

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // Create rectangle that has 100% of the width of the parent and take half the height of the parent
    let rect1 = ScaledRect::new(100f32, 50f32, vec![], Color::RGBA(100, 0, 0, 0));
    let rect2 = ScaledRect::new(100f32, 50f32, vec![], Color::RGBA(0, 100, 0, 0));
    let rect3 = ScaledRect::new(100f32, 50f32, vec![], Color::RGBA(0, 0, 100, 0));
    let rect4 = ScaledRect::new(100f32, 50f32, vec![], Color::RGBA(0, 100, 100, 0));

    // Create a column that contains the two boxes
    let col1 = Flex::new(vec![Box::new(rect1), Box::new(rect2)], Size::new(50, 100), 0, Direction::Column);
    let col2 = Flex::new(vec![Box::new(rect3), Box::new(rect4)], Size::new(50, 100),  0, Direction::Column);
    // Create a row to contain the two columns (making a grid);
    let mut main_row = Flex::new(vec![Box::new(col1), Box::new(col2)], Size::new(100, 100),  0, Direction::Row);

    // Box container, containing the row
    let mut main = ScaledRect::new(100f32, 100f32, vec![Box::new(main_row)], Color::RGBA(18, 18, 18, 0));
    main.set_margin(Position(25, 25));

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
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

        main.render(size, &mut canvas, None);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

