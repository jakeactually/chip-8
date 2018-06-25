use rand;

pub struct Cpu {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,    
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: Vec<u16>,
    pub halted: bool,
    pub hack: Hack
}

pub enum Message {
    Clear,
    Draw(u8, u8, u8),
    Keys(u8, u8, u8),
    GetKey(u8),
    NoMessage
}

pub struct Hack {
    pub shift_hack: bool,
    pub memory_hack: bool
}

impl Cpu {
    pub fn new(hack: Hack) -> Cpu {
        Cpu {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 512,
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![],
            halted: false,
            hack
        }
    }

    pub fn step(&mut self) -> Message {       
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[self.pc as usize + 1];
        self.pc += 2;

        let bits1 = byte1 >> 4 & 15;
        let bits2 = byte1 & 15 ;
        let bits3 = byte2 >> 4 & 15;
        let bits4 = byte2 & 15;

        // units

        match bits1 {
            0x1 => self.jump(bits2, bits3, bits4),
            0x2 => self.call_subroutine(bits2, bits3, bits4),
            0x3 => self.equal_to(bits2, bits3, bits4),
            0x4 => self.not_equal_to(bits2, bits3, bits4),
            0x5 => self.equal(bits2, bits3),
            0x6 => self.variable(bits2, bits3, bits4),
            0x7 => self.add_variable(bits2, bits3, bits4),
            0x8 => self.operations(bits2, bits3, bits4),
            0x9 => self.not_equal(bits2, bits3),
            0xA => self.memory(bits2, bits3, bits4),
            0xB => self.jump_plus(bits2, bits3, bits4),
            0xC => self.random(bits2, bits3, bits4),
            0xF => {
                if byte2 != 0x0A {
                    self.others(bits2, bits3, bits4)
                }
            },
            _ => ()
        }

        // messages

        match bits1 {
            0x0 => self.clear_and_return(bits2, bits3, bits4),
            0xD => Message::Draw(bits2, bits3, bits4),
            0xE => Message::Keys(bits2, bits3, bits4),
            0xF => {
                if byte2 == 0x0A {
                    // FX0A 	KeyOp 	Vx = get_key()
                    // A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
                    Message::GetKey(bits2)
                } else {
                    Message::NoMessage
                }
            }
            _ => Message::NoMessage
        }
    }

    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }        
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn clear_and_return(&mut self, bits2: u8, bits3: u8, bits4: u8) -> Message {
        let three_last = (bits2 as u16) << 8 | (bits3 as u16) << 4 | bits4 as u16;
        match three_last {
            // 00E0 	Display 	disp_clear() 	Clears the screen.
            0x0E0 => Message::Clear,
            // 00EE 	Flow 	return; 	Returns from a subroutine.
            0x0EE => {
                self.pc = self.stack.pop().unwrap();
                Message::NoMessage
            },
            _ => Message::NoMessage
        }
    }

    // 1NNN 	Flow 	goto NNN; 	Jumps to address NNN.
    pub fn jump(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let three_last = (bits2 as u16) << 8 | (bits3 as u16) << 4 | bits4 as u16;
        self.pc = three_last;
    }

    // 2NNN 	Flow 	*(0xNNN)() 	Calls subroutine at NNN.
    pub fn call_subroutine(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let three_last = (bits2 as u16) << 8 | (bits3 as u16) << 4 | bits4 as u16;
        self.stack.push(self.pc);
        self.pc = three_last;
    }

    // 3XNN 	Cond 	if(Vx==NN)
    // Skips the next instruction if VX equals NN.
    // (Usually the next instruction is a jump to skip a code block)
    pub fn equal_to(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let two_last = bits3 << 4 | bits4;
        if self.v[bits2 as usize] == two_last {
            self.pc += 2;
        }
    }

    // 4XNN 	Cond 	if(Vx!=NN)
    // Skips the next instruction if VX doesn't equal NN.
    // (Usually the next instruction is a jump to skip a code block)
    pub fn not_equal_to(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let two_last = bits3 << 4 | bits4;
        if self.v[bits2 as usize] != two_last {
            self.pc += 2;
        }
    }

    // 5XY0 	Cond 	if(Vx==Vy)
    // Skips the next instruction if VX equals VY.
    // (Usually the next instruction is a jump to skip a code block)
    pub fn equal(&mut self, bits2: u8, bits3: u8) {
        if self.v[bits2 as usize] == self.v[bits3 as usize] {
            self.pc += 2;
        }
    }

    // 6XNN 	Const 	Vx = NN 	Sets VX to NN.
    pub fn variable(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let two_last = bits3 << 4 | bits4;
        self.v[bits2 as usize] = two_last;
    }

    // 7XNN 	Const 	Vx += NN 	Adds NN to VX. (Carry flag is not changed)
    pub fn add_variable(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let two_last = bits3 << 4 | bits4;
        self.v[bits2 as usize] = self.v[bits2 as usize].wrapping_add(two_last);
    }

    pub fn operations(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let v = &mut self.v;
        let x = bits2 as usize;
        let y = bits3 as usize;

        match bits4 {
            // 8XY0 	Assign 	Vx=Vy 	Sets VX to the value of VY.
            0x0 => v[x] = v[y],

            // 8XY1 	BitOp 	Vx=Vx|Vy 	Sets VX to VX or VY. (Bitwise OR operation)
            0x1 => v[x] |= v[y],

            // 8XY2 	BitOp 	Vx=Vx&Vy 	Sets VX to VX and VY. (Bitwise AND operation)
            0x2 => v[x] &= v[y],

            // 8XY3 	BitOp 	Vx=Vx^Vy 	Sets VX to VX xor VY.
            0x3 => v[x] ^= v[y],

            // 8XY4 	Math 	Vx += Vy
            // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
            0x4 => {
                v[x] = v[x].wrapping_add(v[y]);
                v[15] = if v[x] < v[y] { 1 } else { 0 };
            },

            // 8XY5 	Math 	Vx -= Vy
            // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            0x5 => {
                v[15] = if v[y] > v[x] { 0 } else { 1 };
                v[x] = v[x].wrapping_sub(v[y]);
            },

            // 8XY6 	BitOp 	Vx=Vy=Vy>>1
            // Shifts VY right by one and copies the result to VX.
            // VF is set to the value of the least significant bit of VY before the shift.
            0x6 => {
                if self.hack.shift_hack {
                    v[15] = v[x] & 1;
                    v[x] >>= 1;
                } else {
                    v[15] = v[y] & 1;
                    v[x] = v[y] >> 1;
                }
            },

            // 8XY7 	Math 	Vx=Vy-Vx
            // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            0x7 => {
                v[15] = if v[x] > v[y] { 0 } else { 1 };
                v[x] = v[y].wrapping_sub(v[x]);
            },

            // 8XYE 	BitOp 	Vx=Vy=Vy<<1
            // Shifts VY left by one and copies the result to VX.
            // VF is set to the value of the most significant bit of VY before the shift.
            0xE => {
                if self.hack.shift_hack {
                    v[15] = v[x] >> 7;
                    v[x] <<= 1;
                } else {
                    v[15] = v[y] >> 7;
                    v[x] = v[y] << 1;
                }
            },
            _ => ()
        }
    }

    // 9XY0 	Cond 	if(Vx!=Vy)
    // Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
    pub fn not_equal(&mut self, bits2: u8, bits3: u8) {
        if self.v[bits2 as usize] != self.v[bits3 as usize] {
            self.pc += 2;
        }
    }

    // ANNN 	MEM 	I = NNN 	Sets I to the address NNN.
    pub fn memory(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let three_last = (bits2 as u16) << 8 | (bits3 as u16) << 4 | bits4 as u16;
        self.i = three_last;
    }

    // BNNN 	Flow 	PC=V0+NNN 	Jumps to the address NNN plus V0.
    pub fn jump_plus(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let three_last = (bits2 as u16) << 8 | (bits3 as u16) << 4 | bits4 as u16;
        self.pc = self.v[0] as u16 + three_last;
    }

    // CXNN 	Rand 	Vx=rand()&NN
    // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
    pub fn random(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let two_last = bits3 << 4 | bits4;
        let random = rand::random::<u8>();
        self.v[bits2 as usize] = random & two_last;
    }

    pub fn others(&mut self, bits2: u8, bits3: u8, bits4: u8) {
        let two_last = bits3 << 4 | bits4;
        let v = &mut self.v;
        let x = bits2 as usize;

        match two_last {
            // FX07 	Timer 	Vx = get_delay() 	Sets VX to the value of the delay timer.
            0x07 => v[x] = self.delay_timer,

            // FX15 	Timer 	delay_timer(Vx) 	Sets the delay timer to VX.
            0x15 => self.delay_timer = v[x],

            // FX18 	Sound 	sound_timer(Vx) 	Sets the sound timer to VX.
            0x18 => self.sound_timer = v[x],

            // FX1E 	MEM 	I +=Vx 	Adds VX to I.[3]
            0x1E => {
                self.i += v[x] as u16;
                v[15] = if self.i >= 4096 { 1 } else { 0 };
            }

            // FX29 	MEM 	I=sprite_addr[Vx]
            // Sets I to the location of the sprite for the character in VX.
            // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
            0x29 => self.i = v[x] as u16 * 5,

            // FX33 	BCD 	set_BCD(Vx); *(I+0)=BCD(3); *(I+1)=BCD(2); *(I+2)=BCD(1);
	        // Stores the binary-coded decimal representation of VX,
            // with the most significant of three digits at the address in I,
            // the middle digit at I plus 1, and the least significant digit at I plus 2.
            // (In other words, take the decimal representation of VX,
            // place the hundreds digit in memory at location in I,
            // the tens digit at location I+1,
            // and the ones digit at location I+2
            0x33 => {
                self.memory[self.i as usize]     = v[x] / 100;
                self.memory[self.i as usize + 1] = v[x] % 100 / 10;
                self.memory[self.i as usize + 2] = v[x] % 10;
            },

            // FX55 	MEM 	reg_dump(Vx,&I)
            // Stores V0 to VX (including VX) in memory starting at address I. I is increased by 1 for each value written.
            0x55 => {
                for j in 0..bits2 + 1 {
                    self.memory[self.i as usize] = v[j as usize];
                    self.i += 1;
                }
                if self.hack.memory_hack {
                    self.i -= bits2 as u16 + 1;
                }
            },
            0x65 => {
                // FX65 	MEM 	reg_load(Vx,&I)
                // Fills V0 to VX (including VX) with values from memory starting at address I. I is increased by 1 for each value written.
                for j in 0..bits2 + 1 {
                    v[j as usize] = self.memory[self.i as usize];
                    self.i += 1;
                }
                if self.hack.memory_hack {
                    self.i -= bits2 as u16 + 1;
                }
            }
            _ => ()
        }
    }
}
