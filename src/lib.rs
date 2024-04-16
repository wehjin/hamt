use std::ops::Index;

use crate::array_map::ElementMap;
use crate::item_store::{ItemRef, ItemStore};
use crate::traits::HamtKey;

#[cfg(test)]
mod tests {}

pub mod array_map;
pub mod datom;
pub mod item_store;
pub mod kv_store;
pub mod traits;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Trie<K: Clone, V: Clone> {
	pub map: ElementMap,
	pub elements: ItemRef<ElementList<K, V>>,
}

impl<K: Clone, V: Clone> Trie<K, V> {
	pub fn size(&self) -> usize {
		let mut count = 0;
		let mut tries = vec![self];
		while let Some(trie) = tries.pop() {
			for element in trie.elements.as_ref().elements() {
				match element {
					Element::KeyValue(_, _) => {
						count += 1;
					}
					Element::SubTrie(_) => {
						todo!()
					}
				}
			}
		}
		count
	}

	pub fn find_element(&self, key_byte: u8) -> Option<&Element<K, V>> {
		self.map
			.to_viewing_index(key_byte)
			.map(|index| &self.elements.as_ref()[index])
	}
}

impl<K: HamtKey + Clone, V: Clone> Trie<K, V> {
	pub fn insert_value(&self, key: K, value: V, store: &mut ItemStore<ElementList<K, V>>) -> Self {
		let depth = 0;
		let trie = self;
		let back_trie: Trie<K, V>;
		loop {
			let key_byte = key.key_byte(depth);
			match self.find_element(key_byte) {
				None => {
					let element = Element::KeyValue(key.clone(), value.clone());
					back_trie = trie.insert_or_replace_element(key_byte, element, store);
					break;
				}
				Some(_) => {
					unimplemented!("existing element")
				}
			}
		}
		back_trie
	}

	fn insert_or_replace_element(&self, key_byte: u8, element: Element<K, V>, store: &mut ItemStore<ElementList<K, V>>) -> Self {
		match self.map.to_viewing_index(key_byte) {
			None => {
				let insertion_index = self.map.to_insertion_index(key_byte);
				let elements = store.push(self.elements.as_ref().insert(insertion_index, element));
				let map = self.map.include_key(key_byte);
				Self { map, elements }
			}
			Some(_) => {
				todo!();
			}
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ElementList<K: Clone, V: Clone>(pub Vec<Element<K, V>>);

impl<K: Clone, V: Clone> ElementList<K, V> {
	pub fn empty() -> Self {
		Self(vec![])
	}

	pub fn insert(&self, index: usize, element: Element<K, V>) -> Self {
		let mut new_elements = self.0.clone();
		new_elements.insert(index, element);
		Self(new_elements)
	}

	pub fn elements(&self) -> &Vec<Element<K, V>> { &self.0 }
}

impl<K: Clone, V: Clone> Index<usize> for ElementList<K, V> {
	type Output = Element<K, V>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element<K: Clone, V: Clone> {
	KeyValue(K, V),
	SubTrie(Trie<K, V>),
}
