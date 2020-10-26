use std::convert::TryInto;

const MEM_SIZE : usize = 4096; //usize
const REG_COUNT : usize = 16;
const STACK_COUNT : usize = 16;

￼ // 0x200

pub struct Chip8 {
	mem:	[u8; MEM_SIZE],
	stack:	[u8; STACK_COUNT],
	v: 	[u8; REG_COUNT],
	i: 	u16,
	pc: 	u16,
	sp: 	u8
}

impl Chip8 {
	pub fn new() -> Chip8 {
		Chip8 {
			mem:	[0; MEM_SIZE],
			stack:	[0; STACK_COUNT],
			v: 	[0; REG_COUNT],
			i: 	0,
			pc: 	0,
			sp: 	0
		}
	}

	pub fn load_rom(&mut self, rom: Vec<u8>) {
		// TODO: Copy from vec to array using slices, check to make sure rom len isnt too large
		//rom.copy_from_slice(&self.memory[ROM_START..ROM_START+rom.len()]);
		//let mem_slice = self.memory[ROM_START..ROM_START+rom.len()];
		//mem_slice.copy_from_slice(&rom[..]);
		//self.memory.copy_from_slice(&rom[]);

		for i in 0..rom.len() {
			self.mem[ROM_START + i] = rom[i];
		}
		self.pc = ROM_START.try_into().unwrap();

		for i in ROM_START..MEM_SIZE-ROM_START {
			print!("{} ", self.mem[i]);
		}

	}

	//pub fn next_instr(&mut self) {

	//}
}
