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

impl<K: HamtKey, V: Clone> Trie<K, V> {
	pub fn find(&self, search_key: &K) -> Option<&V> {
		let mut depth = 0;
		let mut active_trie = self;
		loop {
			let key_byte = search_key.key_byte(depth);
			match active_trie.map.to_viewing_index(key_byte) {
				None => {
					return None;
				}
				Some(viewing_index) => {
					let element_list = active_trie.elements.as_ref();
					let element = &element_list[viewing_index];
					match element {
						Element::KeyValue(key, value) => {
							return (key == search_key).then_some(value);
						}
						Element::SubTrie(trie) => {
							active_trie = trie;
							depth += 1;
						}
					}
				}
			}
		}
	}
	pub fn zip_values(start_depth: usize, (key1, value1): (&K, &V), (key2, value2): (K, V), store: &mut ItemStore<ElementList<K, V>>) -> Self {
		let mut depth = start_depth;
		let mut back_trie: Self;
		let mut back_tasks = Vec::new();
		loop {
			let (key1_byte, key2_byte) = (key1.key_byte(depth), key2.key_byte(depth));
			if key1_byte != key2_byte {
				let map = ElementMap::just_key(key1_byte).include_key(key2_byte);
				let elements = {
					let key1_element = Element::KeyValue(key1.clone(), value1.clone());
					let key2_element = Element::KeyValue(key2, value2);
					let element_list = if key1_byte < key2_byte {
						ElementList::empty().insert(0, key1_element).insert(1, key2_element)
					} else {
						ElementList::empty().insert(0, key2_element).insert(1, key1_element)
					};
					store.push(element_list)
				};
				back_trie = Self { map, elements };
				break;
			} else {
				back_tasks.push(key1_byte);
				depth += 1;
			}
		}
		while let Some(key_byte) = back_tasks.pop() {
			let map = ElementMap::just_key(key_byte);
			let elements = {
				let key_element = Element::SubTrie(back_trie);
				let element_list = ElementList::empty().insert(0, key_element);
				store.push(element_list)
			};
			back_trie = Self { map, elements };
		}
		back_trie
	}

	pub fn insert_value(&self, insert_key: K, insert_value: V, store: &mut ItemStore<ElementList<K, V>>) -> Self {
		let depth = 0;
		let trie = self;
		let back_trie: Trie<K, V>;
		loop {
			let key_byte = insert_key.key_byte(depth);
			let viewing_index = self.map.to_viewing_index(key_byte);
			match viewing_index {
				None => {
					let element = Element::KeyValue(insert_key.clone(), insert_value.clone());
					back_trie = trie.insert_or_replace_element(key_byte, element, store);
					break;
				}
				Some(viewing_index) => {
					match &self.elements.as_ref()[viewing_index] {
						Element::KeyValue(old_key, old_value) => {
							if old_key == &insert_key {
								let replacement = Element::KeyValue(insert_key.clone(), insert_value);
								back_trie = trie.insert_or_replace_element(key_byte, replacement, store);
								break;
							} else {
								let new_map = self.map.clone();
								let new_elements = {
									let zipped_trie = Trie::zip_values(
										depth + 1,
										(old_key, old_value),
										(insert_key, insert_value),
										store,
									);
									let element = Element::SubTrie(zipped_trie);
									let element_list = self.elements.as_ref().replace(viewing_index, element);
									store.push(element_list)
								};
								back_trie = Trie { map: new_map, elements: new_elements };
								break;
							}
						}
						Element::SubTrie(_) => {
							todo!("existing SubTrie")
						}
					}
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
			Some(index) => {
				let elements = store.push(self.elements.as_ref().replace(index, element));
				let map = self.map.clone();
				Self { map, elements }
			}
		}
	}
}

impl<K: Clone, V: Clone> Trie<K, V> {
	pub fn find_element(&self, key_byte: u8) -> Option<&Element<K, V>> {
		let viewing_index = self.map.to_viewing_index(key_byte);
		match viewing_index {
			None => None,
			Some(index) => Some(&self.elements.as_ref()[index])
		}
	}

	pub fn size(&self) -> usize {
		let mut count = 0;
		let mut tries = vec![self];
		while let Some(trie) = tries.pop() {
			for element in trie.elements.as_ref().elements() {
				match element {
					Element::KeyValue(_, _) => {
						count += 1;
					}
					Element::SubTrie(trie) => {
						tries.push(trie);
					}
				}
			}
		}
		count
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

	pub fn replace(&self, index: usize, element: Element<K, V>) -> Self {
		let mut new_elements = self.0.clone();
		new_elements.remove(index);
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
