use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{FileExt, OpenOptionsExt};
use std::path::Path;

use crate::{u32_from_bytes, u32_to_bytes};
use crate::item_stash::element::ElementStoreIndex;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TrieStashIndex(pub u32);

impl TrieStashIndex {
	const INDEX_BYTES: u64 = 4;
	pub fn to_file_index(&self) -> u64 {
		self.0 as u64 * Self::INDEX_BYTES
	}

	pub fn from_file_index(file_index: u64) -> Self {
		let index = (file_index / Self::INDEX_BYTES) as u32;
		Self(index)
	}
}

pub struct TrieStash {
	file: File,
}

impl TrieStash {
	pub fn append(&mut self, element_index: ElementStoreIndex) -> io::Result<TrieStashIndex> {
		let stash_index = self.next_index()?;
		let file_index = stash_index.to_file_index();
		let bytes = u32_to_bytes(element_index.0);
		self.file.write_all_at(&bytes, file_index)?;
		Ok(stash_index)
	}
	fn next_index(&self) -> io::Result<TrieStashIndex> {
		let file_len = self.file.metadata()?.len();
		let stash_index = TrieStashIndex::from_file_index(file_len);
		Ok(stash_index)
	}
	pub fn write(&mut self, index: TrieStashIndex, element_index: ElementStoreIndex) -> io::Result<()> {
		let file_index = index.to_file_index();
		let bytes = u32_to_bytes(element_index.0);
		self.file.write_all_at(&bytes, file_index)?;
		Ok(())
	}
	pub fn read(&self, index: TrieStashIndex) -> io::Result<ElementStoreIndex> {
		let file_index = index.to_file_index();
		let mut bytes = [0u8; 4];
		self.file.read_exact_at(&mut bytes, file_index)?;
		let element_index = ElementStoreIndex(u32_from_bytes(&bytes));
		Ok(element_index)
	}
	pub fn open(stash_path: impl AsRef<Path>) -> io::Result<Self> {
		let file = OpenOptions::new().read(true).write(true).open(stash_path)?;
		let stash = Self { file };
		Ok(stash)
	}

	pub fn create(stash_path: impl AsRef<Path>) -> io::Result<()> {
		OpenOptions::new().write(true).mode(0o600).create_new(true).open(stash_path)?;
		Ok(())
	}
}

