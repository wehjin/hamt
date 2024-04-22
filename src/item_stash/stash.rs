use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::item_stash::element::ElementStoreIndex;
use crate::item_stash::store::ElementStore;

#[derive(Debug)]
pub struct ItemStash {
	store: ElementStore,
}

impl ItemStash {
	pub fn read(&self, position: ElementStoreIndex, index: usize) -> std::io::Result<[u8; 8]> {
		self.store.read(position, index)
	}
	pub fn append(&mut self, elements: impl AsRef<[[u32; 2]]>) -> std::io::Result<ElementStoreIndex> {
		self.store.append(elements)
	}
	pub fn len(&self) -> usize { self.store.len() }
	pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
		let path = path.as_ref().to_path_buf();
		let store = ElementStore::open(store_path(&path))?;
		Ok(Self { store })
	}
	pub fn create(path: impl AsRef<Path>) -> std::io::Result<()> {
		let stash_dir = path.as_ref();
		if stash_dir.exists() {
			return Err(std::io::Error::from(ErrorKind::AlreadyExists));
		}
		fs::create_dir(stash_dir)?;
		ElementStore::create(store_path(stash_dir))?;
		Ok(())
	}
}

fn store_path(stash_path: impl AsRef<Path>) -> PathBuf {
	stash_path.as_ref().join("elements.store")
}
