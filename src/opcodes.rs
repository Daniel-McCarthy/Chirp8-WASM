use display;
use memory;

extern {
	fn invalid_opcode_alert(memory_pointer: u16, opcode: u16);
}

pub unsafe fn select_opcode(opcode: u16)
{
	
	match opcode & 0xF000
	{
		0x0000 => 
		{
			match opcode & 0x0FFF
			{
				0x00E0 => 
				{
					//Clear Screen
					opcode_00e0();
				},
				0x00EE =>
				{
					//Return
					opcode_00ee();
				},
				_ => { invalid_opcode_alert(memory::MEMPOINTER as u16, opcode); },
			}
		},
		0x1000 => 
		{
			//Jump to NNN
			opcode_1nnn(opcode & 0x0FFF);
		},
		0x2000 => 
		{
			//Call subroutine at NNN
			opcode_2nnn(opcode & 0x0FFF);
		},
		0x3000 => 
		{	//Conditional skip (if reg[x] == NN)
			opcode_3xnn(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8);
		},
		0x4000 => 
		{
			//Conditional skip (if reg[x] != NN)
			opcode_4xnn(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8);
		},
		0x5000 => 
		{
			//Conditional skip (if reg[x] == reg[y])
			opcode_5xyn(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
		},
		0x6000 => 
		{
			//Set reg[x] = NN
			opcode_6xnn(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8);
		},
		0x7000 => 
		{
			//Set reg[x] += NN
			opcode_7xnn(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8);
		},
		0x8000 => 
		{
			match opcode & 0x000F
			{
				0x0000 => 
				{
					//Set reg[x] = reg[y]
					opcode_8xy0(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
				},
				0x0001 =>
				{
					//Set reg[x] |= reg[y]
					opcode_8xy1(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
				},
				0x0002 => 
				{
					//Set reg[x] &= reg[y]
					opcode_8xy2(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
				},
				0x0003 => 
				{
					//Set reg[x] ^= reg[y]
					opcode_8xy3(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
				},
				0x0004 => 
				{
					//Set reg[x] += reg[y]
					opcode_8xy4(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
				},
				0x0005 => 
				{
					//Set reg[x] -= reg[y]
					opcode_8xy5(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
				},
				0x0006 => 
				{
					//Set reg[x] >>= 1
					opcode_8xy6(((opcode & 0x0F00) >> 8) as u8);
				},
				0x0007 => 
				{
					//Set reg[x] = reg[y] - reg[x]
					opcode_8xy7(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
				},
				0x000E => 
				{
					//Set reg[x] <<= 1
					opcode_8xye(((opcode & 0x0F00) >> 8) as u8);
				},
				_ => { invalid_opcode_alert(memory::MEMPOINTER as u16, opcode); },
			}
		},
		0x9000 => 
		{
			//Conditional skip (if reg[x] != reg[y])
			opcode_9xy0(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8);
		},
		0xA000 => 
		{
			//Sets I = NNNN
			opcode_annn(opcode & 0x0FFF);
		},
		0xB000 => 
		{
			//Jump to NNNN + reg[0]
			opcode_bnnn(opcode & 0x0FFF);
		},
		0xC000 => 
		{
			//Sets reg[x] = rand() & NN
			opcode_cxnn(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8);
		},
		0xD000 => 
		{
			//Draw sprite to X: reg[x], Y: reg[y], of length N bytes
			opcode_dxyn(((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8, (opcode & 0x000F) as u8); //implement
		},
		0xE000 => 
		{
			match opcode & 0x00FF
			{
				0x009E => 
				{
					//Conditional skip (if key x == reg[x])
					opcode_ex9e(((opcode & 0x0F00) >> 8) as u8);
				},
				0x00A1 =>
				{
					//Conditional skip (if key x != reg[x])
					opcode_exa1(((opcode & 0x0F00) >> 8) as u8);
				},
				_ => { invalid_opcode_alert(memory::MEMPOINTER as u16, opcode); },
			}
		},
		0xF000 => 
		{
			match opcode & 0x00FF
			{
				0x0007 => 
				{
					//Set reg[x] = delayTimer
					opcode_fx07(((opcode & 0x0F00) >> 8) as u8);
				},
				0x000A =>
				{
					//Stop until key input received
					opcode_fx0a(((opcode & 0x0F00) >> 8) as u8);
				},
				0x0015 =>
				{
					//Set delayTimer = reg[x]
					opcode_fx15(((opcode & 0x0F00) >> 8) as u8);
				},
				0x0018 =>
				{
					//Set soundTimer = reg[x]
					opcode_fx18(((opcode & 0x0F00) >> 8) as u8);
				},
				0x001E =>
				{
					//Set I += reg[x]
					opcode_fx1e(((opcode & 0x0F00) >> 8) as u8);
				},
				0x0029 =>
				{
					//Set I to font character X
					opcode_fx29(((opcode & 0x0F00) >> 8) as u8);
				},
				0x0033 =>
				{
					//Store BCD of reg[x] at I
					opcode_fx33(((opcode & 0x0F00) >> 8) as u8);
				},
				0x0055 =>
				{
					//Store reg[0] to reg[x] values at I to I + X
					opcode_fx55(((opcode & 0x0F00) >> 8) as u8);
				},
				0x0065 =>
				{
					//Fill reg[0] to reg[x] with memory at I to I + X
					opcode_fx65(((opcode & 0x0F00) >> 8) as u8);
				},
				_ => { invalid_opcode_alert(memory::MEMPOINTER as u16, opcode); },
			}
		},
		_ => { invalid_opcode_alert(memory::MEMPOINTER as u16, opcode); },
	}
	
	
}

unsafe fn opcode_00e0()
{
	display::clear_screen();
}

unsafe fn opcode_00ee()
{
	//Return
	memory::return_subroutine();
}

unsafe fn opcode_1nnn(nnn: u16)
{
	memory::jump(nnn);
}

unsafe fn opcode_2nnn(nnn: u16)
{
	memory::call_subroutine(nnn);
}

unsafe fn opcode_3xnn(x: u8, nn: u8)
{
	//Conditional skip (if reg[x] == nn)
	if memory::REG[x as usize] == nn {
		memory::MEMPOINTER += 2;
	}
}

unsafe fn opcode_4xnn(x: u8, nn: u8)
{
	//Conditional skip (if reg[x] != nn)
	if memory::REG[x as usize] != nn {
		memory::MEMPOINTER += 2;
	}
}

unsafe fn opcode_5xyn(x: u8, y: u8)
{
	//Conditional skip (if reg[x] == reg[y])
	if memory::REG[x as usize] == memory::REG[y as usize] {
		memory::MEMPOINTER += 2;
	}
}

unsafe fn opcode_6xnn(x: u8, nn: u8)
{
	//Set reg[x] = nn
	memory::REG[x as usize] = nn;
}

unsafe fn opcode_7xnn(x: u8, nn: u8)
{
	//Set reg[x] += nn
	memory::REG[x as usize] = memory::REG[x as usize].wrapping_add(nn);
}

unsafe fn opcode_8xy0(x: u8, y: u8)
{
	//Set reg[x] = reg[y]
	memory::REG[x as usize] = memory::REG[y as usize];
}

unsafe fn opcode_8xy1(x: u8, y: u8)
{
	//Set reg[x] |= reg[y]
	memory::REG[x as usize] |= memory::REG[y as usize];
}

unsafe fn opcode_8xy2(x: u8, y: u8)
{
	//Set reg[x] &= reg[y]
	memory::REG[x as usize] &= memory::REG[y as usize];
}

unsafe fn opcode_8xy3(x: u8, y: u8)
{
	//Set reg[x] ^= reg[y]
	memory::REG[x as usize] ^= memory::REG[y as usize];
}

unsafe fn opcode_8xy4(x: u8, y: u8)
{
	//Set reg[x] += reg[y]
	memory::REG[0xF as usize] = if (memory::REG[x as usize] as u16 + memory::REG[y as usize] as u16) > 0xFF { 1 } else  { 0 };				
	memory::REG[x as usize] = memory::REG[x as usize].wrapping_add(memory::REG[y as usize]);
}

unsafe fn opcode_8xy5(x: u8, y: u8)
{
	//Set reg[x] -= reg[y]
	memory::REG[0xF as usize] = if memory::REG[x as usize] > memory::REG[y as usize] { 1 } else  { 0 };				
	memory::REG[x as usize] = memory::REG[x as usize].wrapping_sub(memory::REG[y as usize]);
}

unsafe fn opcode_8xy6(x: u8)
{
	//Set reg[x] >>= 1
	memory::REG[0xF as usize] = if (memory::REG[x as usize] & 0x1) > 0 { 1 } else  { 0 };					
	memory::REG[x as usize] >>= 1;
}

unsafe fn opcode_8xy7(x: u8, y: u8)
{
	//Set reg[x as usize] = reg[y as usize] - reg[x as usize]
	memory::REG[0xF as usize] = if memory::REG[y as usize] > memory::REG[x as usize] { 1 } else  { 0 };
						
	memory::REG[x as usize] = memory::REG[y as usize].wrapping_sub(memory::REG[x as usize]);
}

unsafe fn opcode_8xye(x: u8)
{
	//Set reg[x] <<= 1
	memory::REG[0xF as usize] = if ((memory::REG[x as usize] & 0x80) >> 7) > 0 { 1 } else  { 0 };
						
	memory::REG[x as usize] <<= 1;
}

unsafe fn opcode_9xy0(x: u8, y: u8)
{
	//Conditional skip (if reg[x] != reg[y])
	if memory::REG[x as usize] != memory::REG[y as usize] {
		memory::MEMPOINTER += 2;
	}
}

unsafe fn opcode_annn(nnn: u16)
{
	//Set I = nnn
	memory::I = nnn as usize;
}

unsafe fn opcode_bnnn(nnn: u16)
{
	memory::jump(nnn + (memory::REG[0 as usize] as u16) );
}

extern {
	fn get_rand() -> u16;
}

unsafe fn opcode_cxnn(x: u8, nn: u8)
{
	//Set reg[x] = rand() & nn
	memory::REG[x as usize] = (get_rand() & (nn as u16)) as u8;					
}

unsafe fn opcode_dxyn(x: u8, y: u8, n: u8) {
	display::draw_sprite(memory::REG[x as usize], memory::REG[y as usize], n);

}

unsafe fn opcode_ex9e(x: u8)
{
	//Skips next instruction if key x is down
	if memory::KEYS[ memory::REG[x as usize ] as usize] {
		memory::MEMPOINTER += 2;
	}				
}

unsafe fn opcode_exa1(x: u8)
{
	//Skips next instruction if key x is not down
	if !(memory::KEYS[ memory::REG[x as usize ] as usize]) {
		memory::MEMPOINTER += 2;
	}				
}

unsafe fn opcode_fx07(x: u8)
{
	//Sets reg[X] to the value of the delay timer
	memory::REG[x as usize] = memory::DELAYTIMER;			
}


unsafe fn opcode_fx0a(x: u8)
{
	//Stop operations until key input stored into reg[X]
	let mut key_found: bool = false;
	
	for i in 0..16 {
		if memory::KEYS[i as usize] {
			key_found = true;
			memory::REG[x as usize] = i;
		}
	}
	
	if !key_found {
		memory::MEMPOINTER -= 2;
	}		
}

unsafe fn opcode_fx15(x: u8)
{
	//Sets the delay timer to reg[X]
	 memory::DELAYTIMER = memory::REG[x as usize];			
}

unsafe fn opcode_fx18(x: u8)
{
	//Sets the sound timer to reg[X]
	 memory::SOUNDTIMER = memory::REG[x as usize];			
}

unsafe fn opcode_fx1e(x: u8)
{
	//Adds VX to I
	 memory::I = memory::I.wrapping_add(memory::REG[x as usize] as usize) as usize;			
}

unsafe fn opcode_fx29(x: u8)
{
	//Set I to position in memory of X Font Character
	memory::I = ((memory::REG[x as usize] as u16).wrapping_mul(5)) as usize;
}

unsafe fn opcode_fx33(x: u8)
{
	//Stores decimal representation of VX at I
	memory::CHIPMEM[memory::I] = memory::REG[x as usize] / 100;
	memory::CHIPMEM[(memory::I + 1 as usize) as usize] = (memory::REG[x as usize] / 10) % 10;
	memory::CHIPMEM[(memory::I + 2 as usize) as usize] = memory::REG[x as usize] % 10;
}


unsafe fn opcode_fx55(x: u8)
{
	//Stores data from registers V0 to VX in memory at address I
	for i in 0..(x+1) { //not sure if x or x+1
		memory::CHIPMEM[(memory::I + i as usize) as usize] = memory::REG[i as usize];
	}
}

unsafe fn opcode_fx65(x: u8)
{
	//Copy data from memory at address I into registers reg[0] to reg[X]
	for i in 0..(x+1) { //not sure if x or x+1
		memory::REG[i as usize] = memory::CHIPMEM[(memory::I + i  as usize) as usize];
	}
}
