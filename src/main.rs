#![allow(warnings, unused, dead_code)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::EventPump;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
const SCALE: u32 = 1;

fn main() {
    let (event_pump, canvas) = create_window();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .unwrap();

    main_loop(event_pump, canvas);
}
fn create_window() -> (EventPump, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("", (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    (event_pump, canvas)
}
fn main_loop(mut event_pump: EventPump, mut canvas: Canvas<Window>) {
    while true {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {}
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {}
                _ => {}
            }
        }
    }
}
