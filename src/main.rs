use std::time::Duration;

use conway::Life;
use sdl2::{keyboard::Keycode, pixels::Color};

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 800;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game of Life", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_scale(16.0, 16.0).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut life = Life::new(SCREEN_WIDTH / 16, SCREEN_HEIGHT / 16);

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

// fn color_cycle(i: u32) -> Color {
//     let i = i % 360;

//     if i < 60 {
//         // orange
//         let i = (i * 256 / 60) as u8;
//         Color::RGB(255, i, 0)
//     } else if i < 120 {
//         // lime
//         let i = ((i - 60) * 256 / 60) as u8;
//         Color::RGB(255 - i, 255, 0)
//     } else if i < 180 {
//         // turquoise
//         let i = ((i - 120) * 256 / 60) as u8;
//         Color::RGB(0, 255, i)
//     } else if i < 240 {
//         // azure
//         let i = ((i - 180) * 256 / 60) as u8;
//         Color::RGB(0, 255 - i, 255)
//     } else if i < 300 {
//         // purple
//         let i = ((i - 240) * 256 / 60) as u8;
//         Color::RGB(i, 0, 255)
//     } else {
//         // pink
//         let i = ((i - 300) * 256 / 60) as u8;
//         Color::RGB(255, 0, 255 - i)
//     }
// }
