use data;
use piston_window::Key;

pub type Gfx = [[u8; 64]; 32];

pub struct Hardware {
    pub gfx: Gfx,
    pub keys: [bool; 17]
}

impl Hardware {
    pub fn press(&mut self, key: Key) {
        self.keys[data::key_to_number(key)] = true;
    }

    pub fn release(&mut self, key: Key) {
        self.keys[data::key_to_number(key)] = false;
    }

    pub fn clear(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                self.gfx[y][x] = 0;
            }
        }
    }
}
