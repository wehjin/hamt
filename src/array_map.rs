#[cfg(test)]
mod tests {
	use std::array;

	use super::*;

	#[test]
	fn keys_to_maps() {
		let keys = [0u8, 1, 7, 8, 30, 31];
		let array_maps: [ElementMap; 6] = array::from_fn(|i| ElementMap::just_key(keys[i]));
		assert_eq!(
			[
				ElementMap(0x00000001),
				ElementMap(0x00000002),
				ElementMap(0x00000080),
				ElementMap(0x00000100),
				ElementMap(0x40000000),
				ElementMap(0x80000000),
			],
			array_maps
		);
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[must_use]
pub struct ElementMap(u32);

impl ElementMap {
	pub fn empty() -> Self { ElementMap(0) }
	pub fn just_key(key: u8) -> Self {
		ElementMap(key_flag(key))
	}
	pub fn include_key(&self, key: u8) -> Self {
		let new_flags = self.0 & key_flag(key);
		Self(new_flags)
	}
	pub fn has_key(&self, key: u8) -> bool {
		let masked = self.0 & key_flag(key);
		masked != 0
	}
	pub fn to_viewing_index(&self, key: u8) -> Option<usize> {
		match self.has_key(key) {
			true => Some(self.to_insertion_index(key)),
			false => None,
		}
	}
	pub fn to_insertion_index(&self, key: u8) -> usize {
		let masked_map = self.0 & count_ones_mask(key);
		masked_map.count_ones() as usize
	}
}

const fn key_flag(key: u8) -> u32 {
	1u32 << key
}

const fn count_ones_mask(key: u8) -> u32 {
	!(0xFFFFFFFFu32 << key)
}