extern crate piston_window;

mod chip8;
mod data;

use chip8::Chip8;
use piston_window::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("", [640, 320])
        .build()
        .unwrap();
    window.window.set_position([(1600 - 640) / 2, (900 - 320) / 2 - 100]);
    
    let chip8arc = Arc::new(Mutex::new(Chip8::new()));
    let chip8clone = chip8arc.clone();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::new(0, 8333333));
            chip8arc.lock().unwrap().cycle();
        }
    });

    while let Some(event) = window.next() {
        let mut chip8 = chip8clone.lock().unwrap();
        if let Some(button_args) = event.button_args() {
            if let Button::Keyboard(key) = button_args.button {
                match button_args.state {
                    ButtonState::Press => chip8.press(key),
                    ButtonState::Release => chip8.release(key),
                }
            }
        }
        window.draw_2d(&event, |context, graphics| {
            chip8.render(&context, graphics);
        });
    }
}
