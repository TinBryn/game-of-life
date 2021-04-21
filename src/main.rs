use std::time::Duration;

use conway::Life;
use sdl2::{keyboard::Keycode, pixels::Color};

const CELL_SIZE: usize = 8;

const BOARD_WIDTH: usize = 100;
const BOARD_HEIGHT: usize = 100;

const SCREEN_WIDTH: usize = BOARD_WIDTH * CELL_SIZE;
const SCREEN_HEIGHT: usize = BOARD_HEIGHT * CELL_SIZE;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game of Life", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas
        .set_scale(CELL_SIZE as f32, CELL_SIZE as f32)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut life = Life::new(BOARD_WIDTH, BOARD_HEIGHT);

    life.randomize();

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => life.randomize(),
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => life.load_from_file("glider_gun").unwrap(),
                _ => {}
            }
        }
        canvas.set_draw_color(Color::WHITE);
        life.draw(&mut canvas).unwrap();
        life = life.next();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
