extern crate piston_window;
extern crate rand;

mod assembler;
mod chip8;
mod cpu;
mod data;
mod dissasembler;
mod hardware;
mod run;

use assembler::assemble;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run::run("roms/Pong (1 player).ch8", true, true);
        return
    }

    if args[1] == "asm" {
        let mut text = String::new();
        let _ = File::open("dev/rom.asm").unwrap().read_to_string(&mut text);
        let _ = File::create("dev/rom.ch8").unwrap().write_all(assemble(text).as_slice());
    }
}
