use crate::key_store::Key;

impl Key for String {
	fn to_shard(&self, depth: usize) -> u8 {
		let full_byte = self.as_bytes()[depth / 2];
		match (depth % 2) == 0 {
			true => full_byte >> 4,
			false => full_byte & 0x0f,
		}
	}
}
