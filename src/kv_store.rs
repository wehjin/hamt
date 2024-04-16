use crate::{ElementList, Trie};
use crate::array_map::ElementMap;
use crate::item_store::ItemStore;
use crate::traits::HamtKey;

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Copy, Clone, Eq, PartialEq)]
	struct TestKey(u8);

	impl HamtKey for TestKey {
		fn key_byte(&self, offset: usize) -> u8 {
			let shift_bits = match offset % 2 {
				0 => 5,
				1 => 0,
				_ => unreachable!("modulo 2")
			};
			self.0 >> shift_bits
		}
	}

	#[test]
	fn insert_same_key_finds_last_value() {
		let mut store = KvStore::open();
		let _ = store.insert_value(TestKey(1), 1);
		let trie = store.insert_value(TestKey(1), 2);
		assert_eq!(1, trie.size());
		assert_eq!(Some(&2), trie.find(&TestKey(1)));
		assert_eq!(None, trie.find(&TestKey(0)));
	}

	#[test]
	fn insert_value_finds_value() {
		let mut store = KvStore::open();
		let trie = store.insert_value(TestKey(1), 1);
		assert_eq!(1, trie.size());
		assert_eq!(Some(&1), trie.find(&TestKey(1)));
		assert_eq!(None, trie.find(&TestKey(0)));
	}
}

pub struct KvStore<K: HamtKey, V: Clone> {
	store: ItemStore<ElementList<K, V>>,
	trie: Trie<K, V>,
}

impl<K: HamtKey, V: Clone> KvStore<K, V> {
	pub fn open() -> Self {
		let mut store = ItemStore::new();
		let trie = Trie {
			map: ElementMap::empty(),
			elements: store.push(ElementList::empty()),
		};
		Self { store, trie }
	}
	pub fn size(&self) -> usize {
		return self.trie.size();
	}

	pub fn insert_value(&mut self, key: K, value: V) -> Trie<K, V> {
		self.trie = self.trie.insert_value(key, value, &mut self.store);
		self.trie.clone()
	}
}
