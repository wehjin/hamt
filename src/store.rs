use std::borrow::Borrow;
use std::ops::{Deref, Index};
use std::rc::Rc;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let mut store = SegmentedItemStore::new();
		let item = 42;
		let item_ref = store.push(item);
		assert_eq!(&item, item_ref.borrow());
	}
}

pub struct SegmentedItemStore<T> {
	mem_segment: Rc<Segment<T>>,
}

impl<Item> SegmentedItemStore<Item> {
	pub fn new() -> Self {
		Self { mem_segment: Rc::new(Segment::new()) }
	}

	pub fn push(&mut self, item: Item) -> SegmentItemRef<Item> {
		let (segment, offset) = self.mem_segment.clone_add_item(item);
		self.mem_segment = Rc::new(segment);
		SegmentItemRef { pos: SegmentItemPos::Mem(offset), segment: self.mem_segment.clone() }
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SegmentId {
	Mem,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Segment<Item> {
	items: Vec<Rc<Item>>,
}

impl<Item> Segment<Item> {
	pub fn new() -> Self {
		Self { items: Vec::new() }
	}

	fn clone_add_item(&self, item: Item) -> (Segment<Item>, usize) {
		let mut items = self.items.clone();
		let item_index = items.len();
		items.push(item.into());
		(Segment { items }, item_index)
	}
}

impl<Item> Index<usize> for Segment<Item> {
	type Output = Item;
	fn index(&self, index: usize) -> &Self::Output { self.items[index].deref() }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SegmentItemPos {
	Mem(usize)
}

impl SegmentItemPos {
	pub fn segment_index(&self) -> usize {
		match self {
			SegmentItemPos::Mem(index) => *index
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SegmentItemRef<Item> {
	pub segment: Rc<Segment<Item>>,
	pub pos: SegmentItemPos,
}

impl<Item> Borrow<Item> for SegmentItemRef<Item> {
	fn borrow(&self) -> &Item {
		&self.segment[self.pos.segment_index()]
	}
}
