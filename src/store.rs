use std::ops::{Deref, Index};
use std::rc::Rc;

use crate::array_map::ArrayMap;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let mut store = SegmentStore::new();
		let item = Item::KeyValue(Value::String("hey".into()), TxEvent(EntityId(1), Effect::Add));
		let item_ref = store.push(item.clone());
		assert_eq!(item, *item_ref);
	}
}

pub struct SegmentStore<TrieKey, TrieValue> {
	mem_segment: Rc<Segment<TrieKey, TrieValue>>,
}

impl<TrieKey, TrieValue> SegmentStore<TrieKey, TrieValue> {
	pub fn new() -> Self {
		Self { mem_segment: Rc::new(Segment::new()) }
	}

	pub fn push(&mut self, segment_item: Item<TrieKey, TrieValue>) -> ItemRef<TrieKey, TrieValue> {
		let (segment, item_index) = self.mem_segment.with_item(segment_item);
		self.mem_segment = Rc::new(segment);
		ItemRef { item_id: ItemId::Mem(item_index), segment: self.mem_segment.clone() }
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Segment<TrieKey, TrieValue> {
	items: Vec<Rc<Item<TrieKey, TrieValue>>>,
}

impl<TrieKey, TrieValue> Segment<TrieKey, TrieValue> {
	pub fn new() -> Self {
		Self { items: Vec::new() }
	}

	pub fn with_item(&self, item: Item<TrieKey, TrieValue>) -> (Segment<TrieKey, TrieValue>, usize) {
		let mut items = self.items.clone();
		let item_index = items.len();
		items.push(item.into());
		(Segment { items }, item_index)
	}
}

impl<TrieKey, TrieValue> Index<usize> for Segment<TrieKey, TrieValue> {
	type Output = Item<TrieKey, TrieValue>;
	fn index(&self, index: usize) -> &Self::Output { self.items[index].deref() }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ItemId {
	Mem(usize)
}

impl ItemId {
	pub fn segment_index(&self) -> usize {
		match self {
			ItemId::Mem(index) => *index
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ItemRef<TrieKey, TrieValue> {
	item_id: ItemId,
	segment: Rc<Segment<TrieKey, TrieValue>>,
}

impl<TrieKey, TrieValue> Deref for ItemRef<TrieKey, TrieValue> {
	type Target = Item<TrieKey, TrieValue>;

	fn deref(&self) -> &Self::Target {
		&self.segment[self.item_id.segment_index()]
	}
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item<TrieKey, TrieValue> {
	KeyValue(TrieKey, TrieValue),
	Node(TrieNode<TrieKey, TrieValue>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TrieNode<TrieKey, TrieValue> {
	map: ArrayMap,
	elements: Vec<ItemRef<TrieKey, TrieValue>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
	String(String)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EntityId(u32);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Effect { Add, Retract }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TxEvent(EntityId, Effect);


