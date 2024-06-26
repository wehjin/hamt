use std::{fs, io};
use std::cell::OnceCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::item_stash::element::ElementStoreIndex;
use crate::item_stash::element_read::{ElementRead, SavedElementList};
use crate::item_stash::stash::ItemStash;
use crate::key_store::{Key, KeyStore, ReadKey};
use crate::key_store::index::KeyStoreIndex;
use crate::key_store::string::StringKeyStore;
use crate::key_store::u32::U32KeyStore;
use crate::kv_forest::array_data::ElementData;
use crate::trie::{Element, Trie, u32_from_stash_index};

#[cfg(test)]
mod tests;
pub mod array_map;
pub mod array_data;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[must_use]
pub struct RootIndex(ElementStoreIndex);

impl RootIndex {
	pub(crate) fn to_u32(&self) -> u32 { self.0.0 }
}

impl From<u32> for RootIndex {
	fn from(value: u32) -> Self { RootIndex(ElementStoreIndex(value)) }
}

struct SizedKeyStore<K: Key>(Box<dyn KeyStore<K>>);

impl<K: Key> ReadKey<K> for SizedKeyStore<K> {
	fn read_key(&self, index: KeyStoreIndex) -> io::Result<K> { self.0.read_key(index) }
}

impl<K: Key> KeyStore<K> for SizedKeyStore<K> {
	fn write_key(&mut self, key: &K) -> io::Result<KeyStoreIndex> { self.0.write_key(key) }
}

pub struct KvForest<K: Key> {
	element_stash: ItemStash,
	element_read: Rc<ElementRead>,
	key_store: SizedKeyStore<K>,
}

impl KvForest<u32> {
	pub fn open(forest_path: impl AsRef<Path>) -> io::Result<Self> {
		let forest = Self::open_or_create_with_keys_store_builder(
			forest_path,
			|path| U32KeyStore::open(path).map(|ks| SizedKeyStore(Box::new(ks) as Box<dyn KeyStore<u32>>)),
		)?;
		Ok(forest)
	}
}

impl KvForest<String> {
	pub fn open(forest_path: impl AsRef<Path>) -> io::Result<Self> {
		let forest = Self::open_or_create_with_keys_store_builder(
			forest_path,
			|path| StringKeyStore::open(path).map(|ks| SizedKeyStore(Box::new(ks) as Box<dyn KeyStore<String>>)),
		)?;
		Ok(forest)
	}
}

impl<K: Key> KvForest<K> {
	pub fn create(path: impl AsRef<Path>) -> io::Result<()> {
		let forest_path = path.as_ref();
		if forest_path.exists() {
			return Err(io::Error::from(ErrorKind::AlreadyExists));
		}
		fs::create_dir(forest_path)?;
		ItemStash::create(element_stash_path(forest_path))?;
		ItemStash::open(element_stash_path(forest_path))?.append([[0u32, 0u32]])?;
		U32KeyStore::create(key_store_path(forest_path))?;
		Ok(())
	}
	fn open_or_create_with_keys_store_builder(forest_path: impl AsRef<Path>, build_keys_store: impl Fn(&Path) -> io::Result<SizedKeyStore<K>>) -> io::Result<Self> {
		if !forest_path.as_ref().exists() {
			Self::create(&forest_path)?;
		}
		let (element_stash, element_read) = {
			let path = element_stash_path(forest_path.as_ref());
			let stash = ItemStash::open(path)?;
			let read = stash.to_element_read()?;
			(stash, read)
		};
		let key_store = build_keys_store(key_store_path(forest_path).as_path())?;
		let forest = Self { element_stash, element_read: Rc::new(element_read), key_store };
		Ok(forest)
	}
	pub fn add_root(&mut self) -> io::Result<RootIndex> {
		let index = RootIndex(ElementStoreIndex(0));
		Ok(index)
	}
	pub fn find(&self, root_index: RootIndex, search_key: &K) -> Option<u32> {
		let trie = self.trie(root_index).expect("find trie at index");
		trie.find(search_key, &self.key_store).cloned()
	}
	pub fn push(&mut self, root_index: RootIndex, insert_key: K, value: u32) -> io::Result<RootIndex> {
		let trie = self.trie(root_index)?;
		let new_trie = trie.push(insert_key, value, &mut self.key_store);
		let new_root_index = self.save(new_trie)?;
		Ok(RootIndex(new_root_index))
	}
	fn save(&mut self, root_trie: Trie) -> io::Result<ElementStoreIndex> {
		let mut relocation_tasks = Vec::new();
		{
			let mut relocation_search = vec![(0, &root_trie)];
			while let Some((depth, trie)) = relocation_search.pop() {
				if trie.is_data_direct() {
					relocation_tasks.push((depth, trie));
					for element_index in 0..trie.elements.len() {
						let element = trie.elements.try_get(element_index)?;
						match element {
							Element::KeyValue { .. } => {}
							Element::SubTrie(child_trie) => {
								relocation_search.push((depth - 1, child_trie));
							}
						}
					}
				}
			}
			relocation_tasks.sort_by_key(|task| task.0);
		}
		let mut stash_indices = HashMap::<u64, ElementStoreIndex>::new();
		for task_index in 0..relocation_tasks.len() {
			let (_, trie) = relocation_tasks[task_index];
			let mut to_save = Vec::new();
			for element_index in 0..trie.elements.len() {
				let element = trie.elements.try_get(element_index)?;
				to_save.push(match element {
					Element::KeyValue { key, value } => [key.to_u32(), *value],
					Element::SubTrie(child_trie) => {
						let stash_index = match child_trie.elements.to_stash_index() {
							None => stash_indices[&child_trie.to_uid()].0,
							Some(stash_index) => stash_index.0
						};
						[u32_from_stash_index(stash_index), child_trie.map.0]
					}
				});
			}
			let stash_index = self.element_stash.append(to_save.as_slice())?;
			stash_indices.insert(trie.to_uid(), stash_index);
		}

		let map = root_trie.map.clone();
		let elements = ElementData::Indirect(SavedElementList {
			top_index: stash_indices[&root_trie.to_uid()],
			len: root_trie.map.count_ones() as usize,
			element_read: self.element_read.clone(),
			slab: OnceCell::new(),
		});
		let new_trie = Trie { map, elements };
		let saved_stash_index = self.element_stash.append([new_trie.to_u32s()])?;
		Ok(saved_stash_index)
	}
	fn trie(&self, root_index: RootIndex) -> io::Result<Trie> {
		let root_bytes = self.element_read.read(root_index.0)?;
		let trie = Trie::parse(&root_bytes, self.element_read.clone()).expect("trie root");
		Ok(trie)
	}
}

fn element_stash_path(forest_path: impl AsRef<Path>) -> PathBuf {
	forest_path.as_ref().join("elements.stash")
}

fn key_store_path(forest_path: impl AsRef<Path>) -> PathBuf {
	forest_path.as_ref().join("keys.stash")
}
