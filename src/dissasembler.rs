pub fn disassemble(bytes: Vec<u8>) -> String {
    let size = bytes.len() - 1;
    let mut i = 0;
    let mut text: Vec<String> = vec![];
    while i < size {
        let byte1 = bytes[i];
        let byte2 = bytes[i + 1];
        i += 2;       
        text.push(decode(byte1, byte2));
    }
    text.join("\n")
}

pub fn decode(byte1: u8, byte2: u8) -> String {
    match _x0(byte1) {
        0x0 => match byte2 {
            0xE0 => "CLS".to_string(),
            0xEE => "RET".to_string(),
            _ => "".to_string()
        },
        0x1 => format!("JP {}", _0xxx(byte1, byte2)),
        0x2 => format!("CALL {}", _0xxx(byte1, byte2)),
        0x3 => format!("SEB {}, {}", _0x(byte1), byte2),
        0x4 => format!("SNEB {}, {}", _0x(byte1), byte2),
        0x5 => format!("SE {}, {}", _0x(byte1), _x0(byte2)),
        0x6 => format!("LDB {}, {}", _0x(byte1), byte2),
        0x7 => format!("ADDB {}, {}", _0x(byte1), byte2),
        0x8 => match _0x(byte2) {
            0x0 => format!("LD {}, {}", _0x(byte1), _x0(byte2)),
            0x1 => format!("OR {}, {}", _0x(byte1), _x0(byte2)),
            0x2 => format!("AND {}, {}", _0x(byte1), _x0(byte2)),
            0x3 => format!("XOR {}, {}", _0x(byte1), _x0(byte2)),
            0x4 => format!("ADD {}, {}", _0x(byte1), _x0(byte2)),
            0x5 => format!("SUB {}, {}", _0x(byte1), _x0(byte2)),
            0x6 => format!("SHR {}, {}", _0x(byte1), _x0(byte2)),
            0x7 => format!("SUB {}, {}", _0x(byte1), _x0(byte2)),
            0xE => format!("SHL {}, {}", _0x(byte1), _x0(byte2)),
            _ => "".to_string()
        },
        0x9 => format!("SNE {}, {}", _0x(byte1), _x0(byte2)),
        0xA => format!("LDI {}", _0xxx(byte1, byte2)),
        0xB => format!("JPP {}", _0xxx(byte1, byte2)),
        0xC => format!("RND {}, {}", _0x(byte1), byte2),
        0xD => format!("DRW {}, {}, {}", _0x(byte1), _x0(byte2), _0x(byte2)),
        0xE => match byte2 {
            0x9E => format!("SKP {}", _0x(byte1)),
            0xA1 => format!("SKNP {}", _0x(byte1)),
            _ => "".to_string()
        },
        0xF => match byte2 {
            0x07 => format!("GDT {}", _0x(byte1)),
            0x0A => format!("GK {}", _0x(byte1)),
            0x15 => format!("SDT {}", _0x(byte1)),
            0x18 => format!("SST {}", _0x(byte1)),
            0x1E => format!("ADDI {}", _0x(byte1)),
            0x29 => format!("FONT {}", _0x(byte1)),
            0x33 => format!("BCD {}", _0x(byte1)),
            0x55 => format!("DUMP {}", _0x(byte1)),
            0x65 => format!("LOAD {}", _0x(byte1)),
            _ => "".to_string()
        },
        _ => "".to_string()
    }
}

#[inline]
fn _x0(byte: u8) -> u8 {    
    byte >> 4 & 15
}

#[inline]
fn _0x(byte: u8) -> u8 {
    byte & 15
}

#[inline]
fn _0xxx(byte1: u8, byte2: u8) -> u16 {
    (byte1 as u16) << 8 | byte2 as u16
}
