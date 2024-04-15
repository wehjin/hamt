use std::array;
use std::rc::Rc;

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

mod store;
pub mod array_map;

pub trait HamtKey {
	fn key_byte(&self, offset: usize) -> u8;
}

#[derive(Debug, Clone)]
pub enum HamtArray<K: HamtKey, V> {
	Mem([Option<HamtArrayElement<K, V>>; 32]),
}

impl<K: HamtKey, V> HamtArray<K, V> {
	pub fn new() -> Self {
		HamtArray::Mem(array::from_fn(|_| None))
	}

	pub fn len(&self) -> usize { 0 }
}

#[derive(Debug, Clone)]
pub enum HamtArrayElement<K: HamtKey, V> {
	KeyValue { key: K, value: V },
	SubHamt { sub_hamt: Rc<HamtArray<K, V>> },
}
