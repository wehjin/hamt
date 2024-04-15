#[cfg(test)]
mod tests {
	use std::array;

	use super::*;

	#[test]
	fn keys_to_maps() {
		let keys = [0u8, 1, 7, 8, 30, 31];
		let array_maps: [ArrayMap; 6] = array::from_fn(|i| ArrayMap::from_key(keys[i]));
		assert_eq!(
			[
				ArrayMap(0x00000001),
				ArrayMap(0x00000002),
				ArrayMap(0x00000080),
				ArrayMap(0x00000100),
				ArrayMap(0x40000000),
				ArrayMap(0x80000000),
			],
			array_maps
		);
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayMap(u32);

impl ArrayMap {
	pub fn from_key(key: u8) -> Self {
		ArrayMap(1u32 << key)
	}
}


