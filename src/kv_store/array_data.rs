use std::fmt::Debug;
use std::io;
use std::ops::Index;

use crate::{DirectElementList, Element, ElementList};
use crate::item_stash::element::ElementStoreIndex;
use crate::item_stash::element_read::SavedElementList;

#[derive(Debug, Clone, Hash)]
pub enum ElementData {
	Direct(DirectElementList),
	Indirect(SavedElementList),
}

impl ElementData {
	pub fn insert(&self, index: usize, element: Element) -> Self {
		let new_element_list = match self {
			ElementData::Direct(direct) => direct.insert(index, element),
			ElementData::Indirect(indirect) => indirect.insert(index, element),
		};
		Self::Direct(new_element_list)
	}
	pub fn replace(&self, index: usize, element: Element) -> Self {
		let new_element_list = match self {
			ElementData::Direct(direct) => direct.replace(index, element),
			ElementData::Indirect(indirect) => indirect.replace(index, element),
		};
		Self::Direct(new_element_list)
	}
	pub fn try_get(&self, index: usize) -> io::Result<&Element> {
		match self {
			ElementData::Direct(direct) => direct.try_get(index),
			ElementData::Indirect(indirect) => indirect.try_get(index)
		}
	}
	pub fn len(&self) -> usize {
		match self {
			ElementData::Direct(direct) => direct.0.len(),
			ElementData::Indirect(indirect) => indirect.len,
		}
	}
	pub fn is_direct(&self) -> bool {
		match self {
			ElementData::Direct(_) => true,
			ElementData::Indirect(_) => false,
		}
	}
	pub fn to_stash_index(&self) -> Option<ElementStoreIndex> {
		match self {
			ElementData::Direct(_) => None,
			ElementData::Indirect(indirect) => Some(indirect.top_index),
		}
	}
	pub fn empty() -> Self {
		Self::Direct(DirectElementList::empty())
	}
}

impl Index<usize> for ElementData {
	type Output = Element;

	fn index(&self, index: usize) -> &Self::Output {
		self.try_get(index).expect("get element")
	}
}

