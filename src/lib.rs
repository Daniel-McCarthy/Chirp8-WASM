mod opcodes;
mod display;
mod memory;

use memory::CHIPMEM;
use memory::MEMPOINTER;

extern {
	//fn log_loop(mempointer: u16, opcode: u16, i: u16, reg_x: u8, reg_y: u8);
}

#[no_mangle]
pub unsafe extern fn initialize() {
	memory::load_tetris();
	memory::load_font();
}

#[no_mangle]
pub unsafe extern fn set_key(key: u8, enabled: bool)
{
	memory::KEYS[key as usize] = enabled;
}

#[no_mangle]
pub unsafe extern fn decrement_timers() {
	memory::decrement_timers();
}

#[no_mangle]
pub unsafe extern fn chip_loop()
{
	let opcode: u16 = ((CHIPMEM[MEMPOINTER] as u16) << 8) | CHIPMEM[MEMPOINTER + 1] as u16;
	
	//log_loop(MEMPOINTER as u16, opcode, memory::I as u16, memory::REG[0], memory::REG[1]);
	
	MEMPOINTER+=2;
	opcodes::select_opcode(opcode);
	
	
}