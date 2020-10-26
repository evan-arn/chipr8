use std::fs;
use std::io::Read;
use std::path::Path;
use std::convert::TryInto;

const ROM_START : u16 = 0x200;
const INSTR_LEN : u8 = 0x2;

//TODO: Rename jumps to loc_addr, analyze non-instr data, analyze BXXX instr, draw graph view,
//opcode macro

macro_rules! opcode_mask {
    (base $opcode:expr)  => {($opcode & 0xF000)};
    (addr $opcode:expr)  => ($opcode & 0x0FFF);
    (nnn $opcode:expr)   => ($opcode & 0x0FFF);
    (nn $opcode:expr)    => ((opcode & 0x0FF) as u8);
    (n $opcode:expr)     => ((opcode & 0x00F) as u8);
    (x $opcode:expr)     => (((opcode & 0x0F00) >> 8) as u8);
    (y $opcode:expr)     => (((opcode & 0x00F0) >> 4) as u8);
}

#[derive(Default, Clone)]
struct CodeGroup {
    label: String,
    addrs: Vec<u16>,
    opcodes: Vec<u16>,
    mnemonics: Vec<String>
}

impl CodeGroup {
    pub fn new() -> CodeGroup {
        CodeGroup::default()
    }
}

#[derive(Default)]
pub struct Disassembler {
    bytes: Vec<u8>,
    visited_addrs: Vec<u16>,
    code_groups: Vec<CodeGroup>,
    call_stack: Vec<u16>
}

impl Disassembler {
    pub fn new() -> Disassembler {
        Disassembler::default()
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = fs::File::open(path).unwrap();
        file.read_to_end(&mut self.bytes).unwrap();
    }

    pub fn dissassemble(&mut self) {
        // Traverse over instructions from rom start
        self.recursive_traversal(0x0);

        println!("Rom Size {:X?} ({})", self.bytes.len(), self.bytes.len());
        println!("Code Groups: {}\n", self.code_groups.len());

        for i in &self.code_groups {
            println!("{}", i.label);

            for j in 0..i.addrs.len() {
                println!("{:#05X} | {:04X?} | {}", i.addrs.get(j).unwrap(), i.opcodes.get(j).unwrap(), i.mnemonics.get(j).unwrap());
            }

            println!();
        }
    }

    fn recursive_traversal(&mut self, mut addr: u16) {

        let mut code_group = CodeGroup::new();
        code_group.label = String::from(format!("loc_{:X?}:", addr + ROM_START));

        while addr < self.bytes.len().try_into().unwrap() {
            if self.visited_addrs.contains(&addr) {
                return;
            }

            let opcode = ((self.bytes[addr as usize] as u16) << 8) | self.bytes[addr as usize + 1] as u16;

            self.visited_addrs.push(addr);
            code_group.addrs.push(addr + ROM_START);
            code_group.opcodes.push(opcode);
            code_group.mnemonics.push(self.decode_instruction(addr));

            if Self::is_ret(opcode) {
                self.code_groups.push(code_group);
                let ret_addr = self.call_stack[self.call_stack.len() - 1];
                self.call_stack.pop();
                self.recursive_traversal(ret_addr + INSTR_LEN as u16);
                return;
            }
            else if Self::is_jmp(opcode) {
                self.code_groups.push(code_group);
                let jmp_addr = (opcode & 0x0FFF) - ROM_START;
                self.recursive_traversal(jmp_addr);
                return;
            }
            else if Self::is_call(opcode) {
                self.code_groups.push(code_group);
                let call_addr = (opcode & 0x0FFF) - ROM_START;
                self.call_stack.push(addr);
                self.recursive_traversal(call_addr);
                return;
            }
            else if Self::is_instr_skip(opcode) {
                //self.code_groups.push(code_group.clone());
                addr += INSTR_LEN as u16;
                self.recursive_traversal(addr + 2);
            }
            else {
                addr += INSTR_LEN as u16;
            }
        }
    }

    fn instr_branch_cnt(opcode: u16) -> u8 {
        if opcode == 0x00EE { // RET
            return 1;
        }

        match opcode_mask!(base opcode) {
            0x1000 => 1, // JMP
            0x2000 => 1, // CALL
            0x3000 => 2, // SE  Vx == kk
            0x4000 => 2, // SNE Vx != kk
            0x5000 => 2, // SE  Vx == Vy
            0x9000 => 2, // SNE Vx != Vy
            0xE000 => 2, // SE & SNE if key with vlaue of Vx pressed
            _ => 0
        }
    }

    fn is_ret(opcode: u16) -> bool {
        if opcode == 0x00EE {
            return true;
        }
        return false;
    }

    fn is_jmp(opcode: u16) -> bool {
        if opcode & 0xF000 == 0x1000 {
            return true;
        }
        return false;
    }

    fn is_call(opcode: u16) -> bool {
        if opcode & 0xF000 == 0x2000 {
            return true;
        }
        return false;
    }

    fn is_instr_skip(opcode: u16) -> bool {
        if opcode & 0xF000 == 0x3000 {
            return true;
        }
        else if opcode & 0xF000 == 0x4000 {
            return true;
        }
        else if opcode & 0xF000 == 0x5000 {
            return true;
        }
        else if opcode & 0xF000 == 0x9000 {
            return true;
        }
        else if opcode & 0xF0FF == 0xE09E {
            return true;
        }
        else if opcode & 0xF0FF == 0xE0A1 {
            return true;
        }
        else
        {
            return false;
        }
    }

    fn decode_instruction(&self, address: u16) -> String {
        let opcode = ((self.bytes[address as usize] as u16) << 8) | self.bytes[address as usize + 1] as u16;

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x0FF) as u8;
        let n = (opcode & 0x00F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        let mnemonic = match (opcode & 0xF000) >> 12 {
            0x0 => {
                match opcode & 0x00FF {
                    0x00E0 => String::from("CLS"),
                    0x00EE => String::from("RET"),
                    _      => String::from(format!("SYS {:#01X}", nnn))
                }
            }
            0x1 => String::from(format!("JP {:#01X}", nnn)),
            0x2 => String::from(format!("CALL {:#01X}", nnn)),
            0x3 => String::from(format!("SE V{}, {:X?}", x, nn)),
            0x4 => String::from(format!("SNE V{}, {:X?}", x, nn)),
            0x5 => String::from(format!("SE V{}, V{}", x, y)),
            0x6 => String::from(format!("LD V{:X?}, {}", x, nn)),
            0x7 => String::from(format!("ADD V{}, {:X?}", x, nn)),
            0x8 => {
                match opcode & 0x000F {
                    0x0000 => String::from(format!("LD V{}, V{}", x, y)),
                    0x0001 => String::from(format!("OR V{}, V{}", x, y)),
                    0x0002 => String::from(format!("AND V{}, V{}", x, y)),
                    0x0003 => String::from(format!("XOR V{}, V{}", x, y)),
                    0x0004 => String::from(format!("ADD V{}, V{}", x, y)),
                    0x0005 => String::from(format!("SUB V{}, V{}", x, y)),
                    0x0006 => String::from(format!("SHR V{}, V{}", x, y)),
                    0x0007 => String::from(format!("SUBN V{}, V{}", x, y)),
                    0x000E => String::from(format!("SHL V{}, V{}", x, y)),
                    _ => String::from(format!("Could not Parse {:X?}", opcode))
                }
            }
            0x9 => String::from(format!("SNE V{}, V{}", x, y)),
            0xA => String::from(format!("LD I, {:#01X}", nnn)),
            0xB => String::from(format!("JP V0, {:#01X}", nnn)),
            0xC => String::from(format!("RND V{}, {}", x, nn)),
            0xD => String::from(format!("DRW V{}, V{}, {}", x, y, n)),
            0xE => {
                match opcode & 0x00FF {
                    0x009E => String::from(format!("SKP V{}", x)),
                    0x00A1 => String::from(format!("SKNP V{}", x)),
                    _      => String::from(format!("Could not Parse {:X?}", opcode))
                }
            }
            0xF => {
                match opcode & 0x00FF {
                    0x0007 => String::from(format!("LD V{}, DT", x)),
                    0x000A => String::from(format!("LD V{}, K", x)),
                    0x0015 => String::from(format!("LD DT, V{}", x)),
                    0x0018 => String::from(format!("LD ST, V{}", x)),
                    0x001E => String::from(format!("ADD I, V{}", x)),
                    0x0029 => String::from(format!("LD F, V{}", x)),
                    0x0033 => String::from(format!("LD B, V{}", x)),
                    0x0055 => String::from(format!("LD I, V{}", x)),
                    0x0065 => String::from(format!("LD V{}, I", x)),
                    _ => String::from(format!("Could not Parse {:X?}", opcode))
                }
            }

            _ => String::from(format!("Could not Parse {:X?}", opcode))
        };
        mnemonic
    }
}

/*  Control Flow Instructions:
    
    00EE - Return from a subroutine.
    1nnn - JP addr
    2nnn - CALL addr
    3xkk - SE Vx, byte  Skip next instruction if Vx = kk.
    4xkk - SNE Vx, byte Skip next instruction if Vx != kk.
    5xy0 - SE Vx, Vy    Skip next instruction if Vx = Vy.
    9xy0 - SNE Vx, Vy   Skip next instruction if Vx != Vy.
    Ex9E - SKP Vx       Skip next instruction if key with the value of Vx is pressed.
    ExA1 - SKNP Vx      Skip next instruction if key with the value of Vx is not pressed.
    
    Requires further analysis
    Bnnn - JP V0, addr  Jump to location nnn + V0.

*/
