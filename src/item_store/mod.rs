use std::ops::{Deref, Index};
use std::rc::Rc;

#[cfg(test)]
mod tests;

pub struct ItemStore<T> {
	mem_segment: Rc<Segment<T>>,
}

impl<Item> ItemStore<Item> {
	pub fn new() -> Self {
		Self { mem_segment: Rc::new(Segment::new()) }
	}

	pub fn push(&mut self, item: Item) -> ItemRef<Item> {
		let (segment, offset) = self.mem_segment.clone_add_item(item);
		self.mem_segment = Rc::new(segment);
		ItemRef { pos: ItemPos::Mem(offset), segment: self.mem_segment.clone() }
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
enum ItemPos {
	Mem(usize)
}

impl ItemPos {
	pub fn segment_index(&self) -> usize {
		match self {
			ItemPos::Mem(index) => *index
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ItemRef<Item> {
	segment: Rc<Segment<Item>>,
	pos: ItemPos,
}

impl<Item> AsRef<Item> for ItemRef<Item> {
	fn as_ref(&self) -> &Item {
		&self.segment[self.pos.segment_index()]
	}
}
