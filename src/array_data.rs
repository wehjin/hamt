use std::fmt::Debug;

use crate::{Element, ElementList};
use crate::item_store::ItemRef;
use crate::traits::HamtKey;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ElementData<K: HamtKey, V: Debug + Clone + PartialEq> {
	Direct(Vec<Element<K, V>>),
	Indirect(ItemRef<ElementList<K, V>>),
}
