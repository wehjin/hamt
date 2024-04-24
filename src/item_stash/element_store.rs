use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{FileExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use crate::item_stash::element::{ELEMENT_BYTES, ElementStoreIndex};
use crate::item_stash::element_read::ElementRead;

#[derive(Debug)]
pub struct ElementStore {
	store_path: PathBuf,
	file: File,
	file_length: u64,
}

impl ElementStore {
	pub fn append(&mut self, elements: impl AsRef<[[u32; 2]]>) -> std::io::Result<ElementStoreIndex> {
		let start_position = self.file_length;
		let mut write_position = start_position;
		for element in elements.as_ref() {
			let bytes = bytes_from_element(element);
			match self.file.write_at(&bytes, write_position) {
				Ok(count) => {
					write_position += count as u64;
				}
				Err(error) => {
					if write_position > start_position {
						self.file.set_len(start_position)?;
						return Err(error);
					}
				}
			}
		}
		self.file_length = write_position;
		Ok(ElementStoreIndex::from_file_position(start_position))
	}
	pub fn len(&self) -> usize {
		self.file_length as usize / ELEMENT_BYTES
	}
	pub fn to_element_read(&self) -> io::Result<ElementRead> { ElementRead::open(&self.store_path) }
	pub fn open(store_path: impl AsRef<Path>) -> std::io::Result<Self> {
		let store_path = store_path.as_ref();
		let file = OpenOptions::new().read(true).append(true).open(store_path)?;
		let file_length = file.metadata()?.len();
		Ok(Self { file, file_length, store_path: store_path.to_path_buf() })
	}
	pub fn create(store_path: impl AsRef<Path>) -> std::io::Result<()> {
		OpenOptions::new()
			.write(true)
			.create(true)
			.mode(0o600)
			.open(&store_path)?;
		Ok(())
	}
}

fn bytes_from_element(element: &[u32; 2]) -> [u8; 8] {
	[
		(element[0] >> 24) as u8,
		(element[0] >> 16) as u8,
		(element[0] >> 08) as u8,
		(element[0] >> 00) as u8,
		(element[1] >> 24) as u8,
		(element[1] >> 16) as u8,
		(element[1] >> 08) as u8,
		(element[1] >> 00) as u8,
	]
}
