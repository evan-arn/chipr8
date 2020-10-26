use std::env;
mod disassemble;

fn main() {
	let rom_path = env::args().nth(1).unwrap();
	let mut disassembler = disassemble::Disassembler::new();
	disassembler.load(rom_path);
	disassembler.dissassemble();
}