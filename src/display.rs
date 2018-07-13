use memory;

extern {
	fn drawPixelWrapper(x: u8, y: u8, enabled: bool);
	fn clearScreenWrapper();
}

pub static mut PIXELS: &'static mut [ bool; 2048] = & mut [false;2048]; //64x32

pub unsafe fn  draw_pixel(x: u8, y: u8, enabled: bool)
{
	drawPixelWrapper(x, y, enabled);
}

pub unsafe fn clear_screen()
{
	clearScreenWrapper();
}

pub unsafe fn xor_pixel(x: u8, y: u8) -> bool {
	let array_position: usize = (x as usize) + (y as usize *64);

	if PIXELS[array_position] {
		draw_pixel(x, y, false);
		PIXELS[array_position] = false;
		true
	}
	else {
		draw_pixel(x, y, true);
		PIXELS[array_position] = true;
		false
	}

}

pub unsafe fn draw_sprite(x: u8, y: u8, n: u8) {

	let mut collision: bool = false;
	
	for i in 0..n {

		let mut sprite_data: u8 = memory::CHIPMEM[memory::I + i as usize];
		
		let mut bit_values: [u8; 8] = [0;8];
		
		for j in 0..8 {  
			bit_values[j] = sprite_data & 0x01;
			sprite_data >>= 1;
		}
		
		bit_values.reverse();
		
		for j in 0..8 {
			if bit_values[j] > 0 {
				if xor_pixel(x.wrapping_add(j as u8), y.wrapping_add(i as u8)) {
					collision = true;
				}
			}
		}
	}
	
	memory::REG[0xF as usize] = if collision { 1 } else { 0 };
}
