use std::io;
use std::path::Path;
use crate::key_store::{Key, KeyStore, ReadKey, u32};
use crate::key_store::index::KeyStoreIndex;
use crate::trie::u32_key_byte;

pub struct U32KeyStore;

impl U32KeyStore {
	pub fn create(_path: impl AsRef<Path>) -> io::Result<()> { Ok(()) }
	pub fn open(_path: impl AsRef<Path>) -> io::Result<Self> {
		Ok(U32KeyStore)
	}
}

impl ReadKey<u32> for U32KeyStore {
	fn read_key(&self, index: KeyStoreIndex) -> io::Result<u32> {
		Ok(index.to_u32())
	}
}

impl KeyStore<u32> for U32KeyStore {
	fn write_key(&mut self, key: &u32) -> io::Result<KeyStoreIndex> {
		Ok(KeyStoreIndex::from(*key))
	}
}

impl Key for u32 {
	fn to_shard(&self, depth: usize) -> u8 {
		u32_key_byte(self, depth)
	}
}

