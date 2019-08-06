extern crate piston_window;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod assembler;
mod chip8;
mod cpu;
mod data;
mod dissasembler;
mod hardware;
mod run;

use assembler::assemble;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File};
use std::io::{Read, Write, stdin};

#[derive(Serialize, Deserialize, Clone)]
struct Game {
    title: String,
    file: String,
    quirks: Option<Quirks>,
    description: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Quirks {
    load_store: Option<bool>,
    shift: Option<bool>
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        play();
    }

    if args[1] == "asm" {
        let mut text = String::new();
        let _ = File::open("dev/rom.asm").unwrap().read_to_string(&mut text);
        let _ = File::create("dev/rom.ch8").unwrap().write_all(assemble(text).as_slice());
    }
}

fn play() {
    let file = File::open("roms/roms.json").unwrap();
    let games: Vec<Game> = serde_json::from_reader(file).unwrap();

    for (i, game) in games.iter().enumerate() {
        println!("{} {}", i, game.title)
    }

    loop {
        ask(&games);
    }
}

fn ask(games: &Vec<Game>) {
    println!();
    println!("choose a game: ");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let index: u32;
    if let Ok(number) = input.trim().parse::<u32>() {
        index = number;
    } else {
        println!("not a number");
        return
    }

    if let Some(game) = games.iter().nth(index as usize) {
        let rom = format!("roms/{}", game.file);
        let quirks = game.clone().quirks;
        let shift = quirks.clone().and_then(|q| q.shift).unwrap_or(false);
        let load_store = quirks.and_then(|q| q.load_store).unwrap_or(false);
        run::run(rom.as_str(), shift, load_store);
    } else {
        println!("not a game");
    }
}
