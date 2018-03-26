use chip8::Chip8;
use piston_window::Key;
use std::process::Command;

pub fn key_to_number(key: Key) -> usize {
    match key {
        Key::D1 => 0,
        Key::D2 => 1,
        Key::D3 => 2,
        Key::D4 => 3,
        Key::Q => 4,
        Key::W => 5,
        Key::E => 6,
        Key::R => 7,
        Key::A => 8,
        Key::S => 9,
        Key::D => 10,
        Key::F => 11,
        Key::Z => 12,
        Key::X => 13,
        Key::C => 14,
        Key::V => 15,
        _ => 16
    }
}

pub fn fontset() -> [u8; 80] {
    [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ]
}

impl Chip8 {
    pub fn debug(&self, byte1: u8, byte2: u8) {
        Command::new("clear").output().unwrap();
        for i in 0..4 {
            for j in 0..4 {
                let index = i * 4 + j;
                print!("v{:x} {:02x} ", index, self.v[index]);
                if j < 3 {
                    print!("| ");
                }
            }
            print!("\n");
        }
        for i in 0..4 {
            for j in 0..4 {
                let index = i * 4 + j;
                let pressed = if self.keys[index] { 1 } else { 0 };
                print!("k{:x} {:02} ", index, pressed);
                if j < 3 {
                    print!("| ");
                }
            }
            print!("\n");
        }
        println!(
            "i {} t {} s {} {:x} \n",
            self.i,
            self.delay_timer,
            self.sound_timer,
            (byte1 as u16) << 8 | byte2 as u16
        );
    }
}

pub fn test() -> Vec<u8> {
    vec![
        0x00,
        0xE0,

        0x60,
        0x02,
        0x61,
        0x02,

        0xEA,
        0xA1,
        0x63,
        0x0A,

        0xEB,
        0xA1,
        0x63,
        0x0B,

        0xF3,
        0x29,
        
        0xD0,
        0x15,

        0x12,
        0x00
    ]
}
