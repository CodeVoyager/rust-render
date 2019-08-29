extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

mod draw;

fn main() {
    let color_white: Color = Color::RGB(255, 255, 255);
    let color_black: Color = Color::RGB(0, 0, 0);
    let color_red: Color = Color::RGB(255, 0, 0);
    let color_blue: Color = Color::RGB(0, 0, 255);
    let color_green: Color = Color::RGB(0, 255, 0);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Rust Renderer",
            (draw::PIXEL_SIZE * draw::WIDTH_WINDOW) as u32,
            (draw::PIXEL_SIZE * draw::HEIGHT_WINDOW) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(color_black);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(color_black);
        canvas.clear();

        draw::pixel(Point::new(0, 0), color_white, &mut canvas);
        draw::line(
            Point::new(20, 2),
            Point::new(40, 30),
            color_red,
            &mut canvas,
        );
        draw::line(
            Point::new(120, 20),
            Point::new(159, 119),
            color_blue,
            &mut canvas,
        );
        draw::triangle(
            draw::Triangle::new(
                Point::new(25, 25),
                Point::new(112, 70),
                Point::new(118, 118),
            ),
            color_green,
            &mut canvas,
        );
        draw::pixel(Point::new(25, 25), color_white, &mut canvas);
        draw::pixel(Point::new(112, 70), color_white, &mut canvas);
        draw::pixel(Point::new(118, 118), color_white, &mut canvas);

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

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
