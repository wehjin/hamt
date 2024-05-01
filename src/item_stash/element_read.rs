use std::cell::OnceCell;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io;
use std::ops::Index;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::item_stash::element::{ELEMENT_BYTES, ElementStoreIndex};
use crate::trie::{Element, ElementList, Trie, u32_from_bytes, u32_to_key};

#[derive(Debug)]
pub struct ElementRead {
	store_path: PathBuf,
	file: File,
}

impl ElementRead {
	pub fn open(store_path: impl AsRef<Path>) -> io::Result<Self> {
		let store_path = store_path.as_ref();
		let file = OpenOptions::new().read(true).open(store_path)?;
		let element_read = Self { file, store_path: store_path.to_path_buf() };
		Ok(element_read)
	}
	pub fn read(&self, index: ElementStoreIndex) -> io::Result<[u8; 8]> {
		let file_index = index.to_file_position();
		let mut bytes = [0u8; ELEMENT_BYTES];
		self.file.read_exact_at(&mut bytes, file_index)?;
		Ok(bytes)
	}
}

impl Hash for ElementRead {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.store_path.hash(state);
	}
}

#[derive(Debug, Clone)]
pub struct SavedElementList {
	pub(crate) top_index: ElementStoreIndex,
	pub(crate) len: usize,
	pub(crate) element_read: Rc<ElementRead>,
	pub(crate) slab: OnceCell<Rc<ElementSlab>>,
}

impl Hash for SavedElementList {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.element_read.hash(state);
		self.top_index.hash(state);
		self.len.hash(state);
	}
}

impl ElementList for SavedElementList {
	fn to_elements(&self) -> Vec<Element> {
		let mut elements = Vec::new();
		for i in 0..self.len {
			let element = self.try_get(i).expect("get element");
			elements.push(element.clone())
		}
		elements
	}

	fn try_get(&self, index: usize) -> io::Result<&Element> {
		let slab = self.slab.get_or_init(|| {
			let slab = ElementSlab::new(self.top_index, self.len as u32, self.element_read.clone()).expect("new slab");
			Rc::new(slab)
		});
		let element = &slab[ElementStoreIndex(self.top_index.0 + index as u32)];
		Ok(element)
	}
}

#[derive(Debug, Clone)]
pub struct ElementSlab {
	top_index: ElementStoreIndex,
	elements: Vec<Element>,
}

impl ElementSlab {
	pub fn new(top_index: ElementStoreIndex, size: u32, element_read: Rc<ElementRead>) -> io::Result<Self> {
		let mut elements = Vec::new();
		{
			let start = top_index.0;
			let end = start + size;
			for i in start..end {
				let bytes = element_read.read(ElementStoreIndex(i))?;
				let element = match Trie::parse(&bytes, element_read.clone()) {
					Some(trie) => Element::SubTrie(trie),
					None => Element::KeyValue {
						key: u32_to_key(u32_from_bytes(&bytes[0..4])),
						value: u32_from_bytes(&bytes[4..8]),
					},
				};
				elements.push(element);
			}
		}
		Ok(Self { top_index, elements })
	}
}

impl Index<ElementStoreIndex> for ElementSlab {
	type Output = Element;

	fn index(&self, index: ElementStoreIndex) -> &Self::Output {
		&self.elements[(index.0 - self.top_index.0) as usize]
	}
}
