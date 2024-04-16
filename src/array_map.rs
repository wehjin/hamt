#[cfg(test)]
mod tests {
	use std::array;

	use super::*;

	#[test]
	fn keys_to_maps() {
		let keys = [0u8, 1, 7, 8, 30, 31];
		let array_maps: [ElementMap; 6] = array::from_fn(|i| ElementMap::from_key(keys[i]));
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
pub struct ElementMap(u32);

impl ElementMap {
	pub fn from_key(key: u8) -> Self {
		ElementMap(1u32 << key)
	}
}


