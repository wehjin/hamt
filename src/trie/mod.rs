use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use std::cell::OnceCell;
use std::io;
use std::ops::Index;
use crate::item_stash::element::ElementStoreIndex;
use crate::item_stash::element_read::{ElementRead, SavedElementList};
use crate::kv_store::array_data::ElementData;
use crate::kv_store::array_map::ElementMap;

#[derive(Debug, Clone, Hash)]
#[must_use]
pub struct Trie {
	pub map: ElementMap,
	pub elements: ElementData,
}

impl Trie {
	pub fn to_uid(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		hasher.finish()
	}
	pub fn is_data_direct(&self) -> bool {
		self.elements.is_direct()
	}
	pub(crate) fn parse(bytes: &[u8; 8], element_read: Rc<ElementRead>) -> Option<Self> {
		let left_u32 = u32_from_bytes(&bytes[0..4]);
		if u32_is_stash_index(left_u32) {
			let map = ElementMap(u32_from_bytes(&bytes[4..8]));
			let elements = ElementData::Indirect(SavedElementList {
				top_index: ElementStoreIndex(left_u32),
				len: map.count_ones() as usize,
				element_read: element_read.clone(),
				slab: OnceCell::new(),
			});
			Some(Trie { map, elements })
		} else {
			None
		}
	}
	pub(crate) fn to_u32s(&self) -> [u32; 2] {
		let left = u32_from_stash_index(self.elements.to_stash_index().expect("stash index").0);
		let right = self.map.0;
		[left, right]
	}
	pub fn find(&self, search_key: &u32) -> Option<&u32> {
		let mut depth = 0;
		let mut active_trie = self;
		loop {
			let key_byte = u32_key_byte(search_key, depth);
			match active_trie.map.to_viewing_index(key_byte) {
				None => {
					return None;
				}
				Some(viewing_index) => {
					let element = active_trie.elements.try_get(viewing_index).expect("get element");
					match element {
						Element::KeyValue { key, value } => {
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
	pub fn push(&self, insert_key: u32, insert_value: u32) -> Self {
		let mut back_trie: Trie;
		let mut back_tasks = Vec::new();
		{
			let mut active_depth = 0;
			let mut active_trie = self;
			loop {
				let key_byte = u32_key_byte(&insert_key, active_depth);
				let viewing_index = active_trie.map.to_viewing_index(key_byte);
				match viewing_index {
					None => {
						let element = Element::KeyValue { key: insert_key.clone(), value: insert_value.clone() };
						back_trie = active_trie.insert_or_replace_element(key_byte, element);
						break;
					}
					Some(viewing_index) => {
						match active_trie.elements.try_get(viewing_index).expect("get element") {
							Element::KeyValue { key: old_key, value: old_value } => {
								if old_key == &insert_key {
									let replacement = Element::KeyValue { key: insert_key.clone(), value: insert_value };
									back_trie = active_trie.insert_or_replace_element(key_byte, replacement);
									break;
								} else {
									let replacement = {
										let zipped_trie = Trie::zip_values(
											active_depth + 1,
											(old_key, old_value),
											(insert_key, insert_value),
										);
										Element::SubTrie(zipped_trie)
									};
									back_trie = active_trie.insert_or_replace_element(key_byte, replacement);
									break;
								}
							}
							Element::SubTrie(sub_trie) => {
								back_tasks.push((key_byte, active_trie));
								active_trie = sub_trie;
								active_depth += 1;
							}
						}
					}
				}
			}
		}
		while let Some((key_byte, trie)) = back_tasks.pop() {
			let element = Element::SubTrie(back_trie);
			back_trie = trie.insert_or_replace_element(key_byte, element);
		}
		back_trie
	}

	fn zip_values(start_depth: usize, (key1, value1): (&u32, &u32), (key2, value2): (u32, u32)) -> Self {
		let mut depth = start_depth;
		let mut back_trie: Self;
		let mut back_tasks = Vec::new();
		loop {
			let (key1_byte, key2_byte) = (u32_key_byte(key1, depth), u32_key_byte(&key2, depth));
			if key1_byte != key2_byte {
				let map = ElementMap::just_key(key1_byte).include_key(key2_byte);
				let elements = {
					let key1_element = Element::KeyValue { key: key1.clone(), value: value1.clone() };
					let key2_element = Element::KeyValue { key: key2, value: value2 };
					let element_list = if key1_byte < key2_byte {
						DirectElementList::empty().insert(0, key1_element).insert(1, key2_element)
					} else {
						DirectElementList::empty().insert(0, key2_element).insert(1, key1_element)
					};
					ElementData::Direct(element_list)
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
				let element_list = DirectElementList::empty().insert(0, key_element);
				ElementData::Direct(element_list)
			};
			back_trie = Self { map, elements };
		}
		back_trie
	}

	fn insert_or_replace_element(&self, key_byte: u8, element: Element) -> Self {
		match self.map.to_viewing_index(key_byte) {
			None => {
				let insertion_index = self.map.to_insertion_index(key_byte);
				let elements = self.elements.insert(insertion_index, element);
				let map = self.map.include_key(key_byte);
				Self { map, elements }
			}
			Some(index) => {
				let elements = self.elements.replace(index, element);
				let map = self.map.clone();
				Self { map, elements }
			}
		}
	}
}

impl Trie {
	pub fn size(&self) -> usize {
		let mut count = 0;
		let mut tries = vec![self];
		while let Some(trie) = tries.pop() {
			for i in 0..trie.elements.len() {
				match &trie.elements[i] {
					Element::KeyValue { .. } => {
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

	pub fn new() -> Self {
		let map = ElementMap::empty();
		let elements = ElementData::empty();
		Self { map, elements }
	}
}

pub trait ElementList {
	fn insert(&self, index: usize, element: Element) -> DirectElementList {
		let mut new_elements = self.to_elements();
		new_elements.insert(index, element);
		DirectElementList(new_elements)
	}
	fn replace(&self, index: usize, element: Element) -> DirectElementList {
		let mut new_elements = self.to_elements();
		new_elements.remove(index);
		new_elements.insert(index, element);
		DirectElementList(new_elements)
	}
	fn to_elements(&self) -> Vec<Element>;

	fn try_get(&self, index: usize) -> io::Result<&Element>;
}

#[derive(Debug, Clone, Hash)]
pub struct DirectElementList(pub Vec<Element>);

impl ElementList for DirectElementList {
	fn to_elements(&self) -> Vec<Element> { self.0.clone() }

	fn try_get(&self, index: usize) -> io::Result<&Element> {
		Ok(&self[index])
	}
}

impl DirectElementList {
	pub fn elements(&self) -> &Vec<Element> { &self.0 }
	pub fn empty() -> Self {
		Self(vec![])
	}
}

impl Index<usize> for DirectElementList {
	type Output = Element;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

#[derive(Debug, Clone, Hash)]
pub enum Element {
	KeyValue { key: u32, value: u32 },
	SubTrie(Trie),
}

impl Element {}

pub fn u32_to_bytes(value: u32) -> [u8; 4] {
	[
		(value >> 24) as u8,
		(value >> 16) as u8,
		(value >> 08) as u8,
		(value >> 00) as u8,
	]
}

pub fn u32_from_bytes(bytes: &[u8]) -> u32 {
	((bytes[0] as u32) << 24)
		+ ((bytes[1] as u32) << 16)
		+ ((bytes[2] as u32) << 8)
		+ (bytes[3] as u32)
}

pub fn u32_key_byte(value: &u32, depth: usize) -> u8 {
	let shifted = match depth % 7 {
		0 => (value >> 30) as u8,
		1 => (value >> 25) as u8,
		2 => (value >> 20) as u8,
		3 => (value >> 15) as u8,
		4 => (value >> 10) as u8,
		5 => (value >> 05) as u8,
		6 => (value >> 00) as u8,
		_ => unreachable!("modulo 7")
	};
	shifted & 0b11111
}

pub fn u32_is_stash_index(value: u32) -> bool {
	(value & 0x80000000) == 0
}

pub fn u32_from_stash_index(stash_index: u32) -> u32 {
	assert_eq!(0, stash_index & 0x80000000);
	stash_index
}

pub fn u32_from_key(key: u32) -> u32 {
	assert_eq!(0, key & 0x80000000);
	key | 0x80000000
}

pub fn u32_to_key(value: u32) -> u32 {
	value & 0x7fffffff
}

