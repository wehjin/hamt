use crate::{ElementList, Trie};
use crate::array_map::ElementMap;
use crate::item_store::ItemStore;
use crate::traits::HamtKey;

#[cfg(test)]
mod tests;

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
