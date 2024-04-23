use std::{fs, io};
use std::fmt::Debug;
use std::ops::Index;
use std::path::Path;

use crate::traits::HamtKey;
use crate::Trie;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KvForestIndex(pub usize);

pub struct KvForest<K: HamtKey, V: Debug + Clone + PartialEq> {
	tries: Vec<Trie<K, V>>,
}

impl<K: HamtKey, V: Debug + Clone + PartialEq> Index<KvForestIndex> for KvForest<K, V> {
	type Output = Trie<K, V>;

	fn index(&self, index: KvForestIndex) -> &Self::Output {
		&self.tries[index.0]
	}
}

impl<K: HamtKey, V: Debug + Clone + PartialEq> KvForest<K, V> {
	pub fn push_trie(&mut self, index: KvForestIndex, key: K, value: V) -> io::Result<()> {
		*self.trie_mut(index) = self[index].insert_value(key, value);
		Ok(())
	}
}

impl<K: HamtKey, V: Debug + Clone + PartialEq> KvForest<K, V> {
	fn trie_mut(&mut self, index: KvForestIndex) -> &mut Trie<K, V> {
		&mut self.tries[index.0]
	}
	pub fn new_trie(&mut self) -> io::Result<KvForestIndex> {
		let trie = Trie::new();
		let index = KvForestIndex(self.tries.len());
		self.tries.push(trie);
		Ok(index)
	}
	pub fn open_or_create(path: impl AsRef<Path>) -> io::Result<Self> {
		Self::create(&path)?;
		Self::open(path)
	}
	pub fn open(_path: impl AsRef<Path>) -> io::Result<Self> {
		let forest = Self { tries: Vec::new() };
		Ok(forest)
	}

	pub fn create(path: impl AsRef<Path>) -> io::Result<()> {
		let dir_path = path.as_ref();
		fs::create_dir_all(dir_path)?;
		Ok(())
	}
}
