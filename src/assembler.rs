pub fn assemble(text: String) -> Vec<u8> {
    let mut opcodes: Vec<u8> = vec![];

    for instruction in parse_instructions(text) {
        let args = instruction.args;
        let opcode  = match instruction.name.as_str() {
            "CLS" => 0x00E0,
            "RET" => 0x00EE,
            "JP" => 0x1000 + args[0],
            "CALL" => 0x2000 + args[0],
            "SEB" => 0x3000 + args[0] * 0x100 + args[1],
            "SNEB" => 0x4000 + args[0] * 0x100 + args[1],
            "SE" => 0x5000 + args[0] * 0x100 + args[1] * 0x10,
            "LDB" => 0x6000 + args[0] * 0x100 + args[1],
            "ADDB" => 0x7000 + args[0] * 0x100 + args[1],
            "LD" => 0x8000 + args[0] * 0x100 + args[1] * 0x10,
            "OR" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0x1,
            "AND" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0x2,
            "XOR" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0x3,
            "ADD" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0x4,
            "SUB" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0x5,
            "SHR" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0x6,
            "SUBN" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0x7,
            "SHL" => 0x8000 + args[0] * 0x100 + args[1] * 0x10 + 0xE,
            "SNE" => 0x9000 + args[0] * 0x100 + args[1] * 0x10,
            "LDI" => 0xA000 + args[0],
            "JPP" => 0xB000 + args[0],
            "RND" => 0xC000 + args[0] * 0x100 + args[1],
            "DRW" => 0xD000 + args[0] * 0x100 + args[1] * 0x10 + args[2],
            "SKP" => 0xE000 + args[0] * 0x100 + 0x9E,
            "SKNP" => 0xE000 + args[0] * 0x100 + 0xA1,
            "GDT" => 0xF000 + args[0] * 0x100 + 0x07,
            "GK" => 0xF000 + args[0] * 0x100 + 0x0A,
            "SDT" => 0xF000 + args[0] * 0x100 + 0x15,
            "SST" => 0xF000 + args[0] * 0x100 + 0x18,
            "ADDI" => 0xF000 + args[0] * 0x100 + 0x1E,
            "FONT" => 0xF000 + args[0] * 0x100 + 0x29,
            "BCD" => 0xF000 + args[0] * 0x100 + 0x33,
            "LOAD" => 0xF000 + args[0] * 0x100 + 055,
            "DUMP" => 0xF000 + args[0] * 0x100 + 0x65,
            _ => 0x0000
        };
        opcodes.push((opcode >> 8) as u8);
        opcodes.push(opcode as u8);
    }

    opcodes
}

struct Instruction {
    name: String,
    args: Vec<u16>
}

fn parse_instructions(text: String) -> Vec<Instruction> {
    text
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| parse_instruction(x.to_string()))
        .collect()
}

fn parse_instruction(line: String) -> Instruction {
    match line.find(' ') {
        Some(index) => {
            let (left, right) = line.split_at(index);
            let name = left.to_string();
            let args = right.split(",").map( |x| x.trim().parse::<u16>().unwrap() ).collect();
            Instruction {
                name,
                args
            }
        },
        None => {
            Instruction {
                name: line,
                args: vec![]
            }
        }
    }    
}
