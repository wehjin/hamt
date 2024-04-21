use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct TestKey(u16);

impl HamtKey for TestKey {
	fn key_byte(&self, offset: usize) -> u8 {
		let shift_bits = match offset % 3 {
			0 => 10,
			1 => 5,
			2 => 0,
			_ => unreachable!("modulo 3")
		};
		((self.0 >> shift_bits) & 0b11111) as u8
	}
}

mod insertion;
mod persistence;