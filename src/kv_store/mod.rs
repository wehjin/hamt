use std::{fs, io};
use std::fmt::Debug;
use std::path::Path;

use crate::{ElementList, Trie};
use crate::array_data::ElementData;
use crate::array_map::ElementMap;
use crate::item_store::ItemStore;
use crate::traits::HamtKey;

#[cfg(test)]
mod tests;

pub struct KvStore<K: HamtKey, V: Debug + Clone + PartialEq> {
	store: ItemStore<ElementList<K, V>>,
	trie: Trie<K, V>,
}

impl<K: HamtKey, V: Debug + Clone + PartialEq> KvStore<K, V> {
	pub fn insert_value(&mut self, key: K, value: V) -> Trie<K, V> {
		self.trie = self.trie.insert_value(key, value, &mut self.store);
		self.trie.clone()
	}
	pub fn size(&self) -> usize {
		return self.trie.size();
	}

	pub fn open_or_create(path: impl AsRef<Path>) -> io::Result<Self> {
		Self::create(&path)?;
		Self::open(path)
	}
	pub fn open(_path: impl AsRef<Path>) -> io::Result<Self> {
		let store = ItemStore::new();
		let trie = Trie {
			map: ElementMap::empty(),
			elements: ElementData::Direct(ElementList::empty()),
		};
		let store = Self { store, trie };
		Ok(store)
	}

	pub fn create(path: impl AsRef<Path>) -> io::Result<()> {
		let dir_path = path.as_ref();
		fs::create_dir_all(dir_path)?;
		Ok(())
	}
}
