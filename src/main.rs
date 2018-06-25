extern crate piston_window;
extern crate rand;

mod chip8;
mod cpu;
mod data;
mod hardware;

use chip8::Chip8;
use cpu::Hack;
use hardware::Gfx;
use piston_window::*;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::string::String;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("", [640, 320])
        .build()
        .unwrap();
    
    window.window.set_position([(1366 - 640) / 2, (768 - 320) / 2 - 100]);
    
    let (sender, receiver) = channel();
    let (sender2, receiver2) = channel();
    
    thread::spawn(move || {
        let hack = Hack { shift_hack: true, memory_hack: true };
        let mut chip8 = Chip8::new(String::from("roms/Blinky [Hans Christian Egeberg, 1991].ch8"), hack);
        let mut key_vx = 0_u8;
        loop {
            if let Ok(button_args) = receiver2.try_recv() {
                chip8.hardware_key(button_args, key_vx);
            }
            chip8.cpu.tick();
            for _ in 0..10 {
                if !chip8.cpu.halted {
                    chip8.debug();

                    use cpu::Message::*;
                    match chip8.step() {                          
                        Draw(_, _, _) => {
                            let _ = sender.send(chip8.hardware.gfx);
                        },
                        GetKey(bits2) => key_vx = bits2,
                        _ => ()
                    }
                }     
            }                  
            thread::sleep(Duration::new(0, 8333333));
        }
    });

    let mut gfx_cache: Gfx = [[0; 64]; 32];
    while let Some(event) = window.next() {
        if let Some(button_args) = event.button_args() {
            let _ = sender2.send(button_args);
        }
        if let Ok(gfx) = receiver.try_recv() {
            gfx_cache = gfx;
        }
        window.draw_2d(&event, |context, graphics| {
            render(&context, graphics, gfx_cache);
        });
    }
}

pub fn render<G>(context: &Context, graphics: &mut G, gfx2: Gfx) where G: Graphics {
    clear(data::BLACK, graphics);
    for y in 0..32 {
        for x in 0..64 {            
            if gfx2[y][x] == 1 {
                let rect = [x as f64 * 10.0, y as f64 * 10.0, 9.0, 9.0];
                rectangle(data::WHITE, rect, context.transform, graphics);
            }
        }
    }
}
