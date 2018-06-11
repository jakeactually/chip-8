extern crate rand;

use cpu::Cpu;
use cpu::Message;
use data;
use hardware::Hardware;
use piston_window::*;
use std::fs::File;
use std::io::Read;
use std::string::String;

pub struct Chip8 {
    pub cpu: Cpu,
    pub hardware: Hardware
}

impl Chip8 {
    pub fn new(path: String) -> Chip8 {
        let mut chip8 = Chip8 {
            cpu: Cpu::new(),
            hardware: Hardware {
                gfx: [[0; 64]; 32],
                keys: [false; 17]
            }
        };
        
        let fontset = data::fontset();
        for i in 0..80 {
            chip8.cpu.memory[i] = fontset[i];
        }

        if path == "test" {
            let test = data::test();
            for (i, &x) in test.iter().enumerate() {
                chip8.cpu.memory[512 + i] = x;
            }            
        } else {
            let rom = File::open(path).unwrap().bytes().take(3896);
            for (i, maybe_byte) in rom.enumerate() {
                chip8.cpu.memory[i + 512] = maybe_byte.unwrap();
            }
        }

        chip8
    }

    pub fn cycle(&mut self) -> Message {
        use cpu::Message::*;
        match self.cpu.cycle() {                          
            Clear => {
                self.hardware.clear();
                NoMessage
            },
            Draw(bits2, bits3, bits4) => {
                self.draw(bits2, bits3, bits4);
                Draw(bits2, bits3, bits4)
            },
            Keys(bits2, bits3, bits4) => {
                self.key(bits2, bits3, bits4);
                NoMessage
            },
            GetKey(bits2) => {                        
                self.cpu.halted = true;
                GetKey(bits2)
            },
            NoMessage => NoMessage
        }
    }
    
    // DXYN 	Disp 	draw(Vx,Vy,N)
    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
    // Each row of 8 pixels is read as bit-coded starting from memory location I;
    // I value doesn’t change after the execution of this instruction.
    // As described above, VF is set to 1
    // if any screen pixels are flipped from set to unset when the sprite is drawn,
    // and to 0 if that doesn’t happen
    pub fn draw(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let mut flipped = 0;
        let x = self.cpu.v[bits2 as usize];
        let y = self.cpu.v[bits3 as usize];
        let height = bits4;

        for relative_y in 0..height {
            let index_y = (y + relative_y) as usize;
            let byte = self.cpu.memory[(self.cpu.i + relative_y as u16) as usize];
            for relative_x in 0..8 {
                let index_x = (x + relative_x) as usize;                
                if index_x < 64 && index_y < 32 {
                    let prev = self.hardware.gfx[index_y][index_x];
                    let next = prev ^ (byte << relative_x & 128) >> 7;
                    if prev == 1 && next == 0 {
                        flipped = 1;
                    }
                    self.hardware.gfx[index_y][index_x] = next;
                }
            }
        }
        self.cpu.v[15] = flipped;
    }
    
    pub fn hardware_key(&mut self, button_args: ButtonArgs, get_key_bits2: u8) {
        if let ButtonArgs { button: Button::Keyboard(key), state, .. } = button_args {
                match state {
                ButtonState::Press => self.hardware.press(key),
                ButtonState::Release => {
                    self.hardware.release(key);
                    if self.cpu.halted {
                        self.cpu.v[get_key_bits2 as usize] = data::key_to_number(key) as u8;
                        self.cpu.halted = false;
                    }
                }
            }
        }        
    }
    
    pub fn key(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let two_last = bits3 << 4 | bits4;
        
        match two_last {
            // EX9E 	KeyOp 	if(key()==Vx)
            // Skips the next instruction if the key stored in VX is pressed.
            // (Usually the next instruction is a jump to skip a code block)
            0x9E => {
                if self.hardware.keys[bits2 as usize] {                    
                    self.cpu.pc += 2;
                }
            },

            // EXA1 	KeyOp 	if(key()!=Vx)
            // Skips the next instruction if the key stored in VX isn't pressed.
            // (Usually the next instruction is a jump to skip a code block)
            0xA1 => {
                if !self.hardware.keys[bits2 as usize] {
                    self.cpu.pc += 2;                    
                }
            },
            _ => ()
        }
    }

    pub fn debug(&self) {
        let byte1 = self.cpu.memory[self.cpu.pc as usize];
        let byte2 = self.cpu.memory[self.cpu.pc as usize + 1];
        
        print!("f ");
        for i in 0..16 {
            print!("{:02x} ", i);            
        }
        print!("\n");

        print!("v ");
        for i in 0..16 {
            print!("{:02x} ", self.cpu.v[i]);            
        }
        print!("\n");

        print!("k ");
        for i in 0..16 {
            print!("{:02x} ", if self.hardware.keys[i] { 1 } else { 0 });            
        }
        print!("\n");

        println!(
            "  o{:04x} p{:04x} k{:04} i{:04x} t{:04x} s{:04x} \n",
            (byte1 as u16) << 8 | byte2 as u16,
            self.cpu.pc,
            self.cpu.stack.len(),
            self.cpu.i,
            self.cpu.delay_timer,
            self.cpu.sound_timer            
        );
    }
}
