use std::{fs, io};
use std::cell::OnceCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::{Element, Trie, u32_from_key, u32_from_stash_index};
use crate::array_data::ElementData;
use crate::item_stash::element::ElementStoreIndex;
use crate::item_stash::element_read::{ElementRead, SavedElementList};
use crate::item_stash::stash::ItemStash;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[must_use]
pub struct RootIndex(ElementStoreIndex);

pub struct KvForest {
	element_stash: ItemStash,
	element_read: Rc<ElementRead>,
}

impl KvForest {
	pub fn push(&mut self, root_index: RootIndex, key: u32, value: u32) -> io::Result<RootIndex> {
		let trie = self.trie(root_index)?;
		let new_trie = trie.push(key, value);
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
					Element::KeyValue { key, value } => [u32_from_key(*key), *value],
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
	pub fn add_root(&mut self) -> io::Result<RootIndex> {
		let index = RootIndex(ElementStoreIndex(0));
		Ok(index)
	}
	pub fn trie(&self, root_index: RootIndex) -> io::Result<Trie> {
		let root_bytes = self.element_read.read(root_index.0)?;
		let trie = Trie::parse(&root_bytes, self.element_read.clone()).expect("trie root");
		Ok(trie)
	}
	pub fn open_or_create(path: impl AsRef<Path>) -> io::Result<Self> {
		if !path.as_ref().exists() {
			Self::create(&path)?;
		}
		Self::open(path)
	}
	pub fn open(forest_path: impl AsRef<Path>) -> io::Result<Self> {
		let element_stash = ItemStash::open(element_stash_path(forest_path.as_ref()))?;
		let element_read = Rc::new(element_stash.to_element_read()?);
		let forest = Self { element_stash, element_read };
		Ok(forest)
	}

	pub fn create(path: impl AsRef<Path>) -> io::Result<()> {
		let forest_path = path.as_ref();
		fs::create_dir_all(forest_path)?;
		ItemStash::create(element_stash_path(forest_path))?;
		ItemStash::open(element_stash_path(forest_path))?.append([[0u32, 0u32]])?;
		Ok(())
	}
}

fn element_stash_path(forest_path: impl AsRef<Path>) -> PathBuf {
	forest_path.as_ref().join("elements.stash")
}
