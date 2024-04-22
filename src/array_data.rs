use std::fmt::Debug;
use std::ops::Index;

use crate::{Element, ElementList};
use crate::item_store::ItemRef;
use crate::traits::HamtKey;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ElementData<K: HamtKey, V: Debug + Clone + PartialEq> {
	Direct(ElementList<K, V>),
	Indirect(ItemRef<ElementList<K, V>>),
}

impl<K: HamtKey, V: Debug + Clone + PartialEq> Index<usize> for ElementData<K, V> {
	type Output = Element<K, V>;

	fn index(&self, index: usize) -> &Self::Output {
		match self {
			ElementData::Direct(direct) => &direct[index],
			ElementData::Indirect(indirect) => &indirect.as_ref()[index],
		}
	}
}

impl<K: HamtKey, V: Debug + Clone + PartialEq> AsRef<ElementList<K, V>> for ElementData<K, V> {
	fn as_ref(&self) -> &ElementList<K, V> {
		match self {
			ElementData::Direct(direct) => direct,
			ElementData::Indirect(indirect) => indirect.as_ref(),
		}
	}
}

impl<K: HamtKey, V: Debug + Clone + PartialEq> ElementData<K, V> {
	pub fn insert(&self, index: usize, element: Element<K, V>) -> Self {
		Self::Direct(self.as_ref().clone().insert(index, element))
	}

	pub fn replace(&self, index: usize, element: Element<K, V>) -> Self {
		Self::Direct(self.as_ref().clone().replace(index, element))
	}

	pub fn elements(&self) -> &Vec<Element<K, V>> {
		self.as_ref().elements()
	}

	pub fn empty() -> Self {
		Self::Direct(ElementList::empty())
	}
}

