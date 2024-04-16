use crate::traits::HamtKey;

#[cfg(test)]
mod tests {
	use crate::HamtKey;

	struct Key(u32);

	impl HamtKey for Key {
		fn key_byte(&self, offset: usize) -> u8 {
			const SHIFT: [u32; 7] = [30, 25, 20, 15, 10, 5, 0];
			(self.0 >> SHIFT[offset % 7]) as u8
		}
	}
}

pub mod array_map;
pub mod datom;
pub mod store;
pub mod traits;
