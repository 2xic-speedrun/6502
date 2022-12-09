pub mod opcodes;
pub mod machine;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    use crate::machine::machine::Machine;
    let input = "a9018d0002";
    let mut machine = Machine::new(input);
    let machine_state = &machine;

    //  Based on the getting started https://docs.rs/sdl2/latest/sdl2/
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        let screen = machine.memory.screen();

        let screen_size = 128;
        let block_size = 8;
        for (i, item) in screen.iter().enumerate() {
                if *item != 0 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    for n in 0..block_size { 
                        for m in 0..block_size { 
                            canvas.draw_point(Point::new(screen_size / 128 + n, (screen_size % 128) + m ));
                        }
                    }
                }
            }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        machine = machine.clone().tick();        

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
