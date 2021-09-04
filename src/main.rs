extern crate sdl2;

mod graphics;

use graphics::element::Element;
use graphics::types::Position;
use graphics::rect::ScaledRect;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut rect1 = ScaledRect::new(100f32, 100f32, vec![], Color::RGBA(100, 0, 0, 0));
    rect1.set_margin(Position(10, 10));


    let mut rect = ScaledRect::new(100f32, 100f32, vec![Box::new(rect1)], Color::RGBA(0, 100, 0, 0));
    rect.set_margin(Position(10, 10));

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

        rect.render(size, &mut canvas, None);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

