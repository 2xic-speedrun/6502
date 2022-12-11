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
//    let input = "a9018d0002a9058d0102a9088d0202a9018d0302a9058d0402a9088d0502"; 
    let input = "200606203806200d06202a0660a9028502a9048503a9118510a9108512a90f8514a90485118513851560a5fe8500a5fe2903186902850160204d06208d0620c306201907202007202d074c3806a5ffc977f00dc964f014c973f01bc961f02260a9042402d026a901850260a9082402d01ba902850260a9012402d010a904850260a9022402d005a9088502606020940620a80660a500c510d00da501c511d007e603e603202a0660a202b510c510d006b511c511f009e8e8e403f0064caa064c350760a603ca8ab5109512ca10f9a5024ab0094ab0194ab01f4ab02fa51038e9208510900160c611a901c511f02860e610a91f2410f01f60a5101869208510b00160e611a906c511f00c60c610a510291fc91ff001604c3507a000a5fe910060a603a9008110a200a901811060a200eaeacad0fb60"; 

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
                if *item == 1 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else if *item == 5 {
                    canvas.set_draw_color(Color::RGB(50, 50, 50));
                } else if *item == 8 {
                    canvas.set_draw_color(Color::RGB(75, 75, 75));
                }
                for n in 0..block_size { 
                    for m in 0..block_size { 
                        let y = ((i as i32) / screen_size) * block_size;
                        let x = ((i as i32) % screen_size) * block_size;
                        canvas.draw_point(Point::new(x + n, y + m ));
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
