use std::io;
use std::path::Path;

use crate::trie::{key_field_from_store_index, key_field_to_store_index, u32_from_bytes, u32_is_stash_index, u32_key_byte};

#[cfg(test)]
mod tests {
	use crate::key_store::{KeyStore, ReadKey, U32KeyStore};

	#[test]
	fn basic() {
		let store = &mut U32KeyStore;
		let write_key = 137u32;
		let key_store_index = store.write_key(&write_key).expect("write_key");
		let read_key = store.read_key(key_store_index).expect("read_key");
		assert_eq!(write_key, read_key);
	}
}

pub trait Key {
	fn to_shard(&self, depth: usize) -> u8;
}

impl Key for u32 {
	fn to_shard(&self, depth: usize) -> u8 {
		u32_key_byte(self, depth)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyField(u32);

impl KeyField {
	pub fn to_u32(&self) -> u32 { self.0 }
}

impl From<KeyStoreIndex> for KeyField {
	fn from(value: KeyStoreIndex) -> Self {
		Self(key_field_from_store_index(value.0))
	}
}

impl From<&[u8]> for KeyField {
	fn from(bytes: &[u8]) -> Self {
		let u32 = u32_from_bytes(bytes);
		debug_assert!(!u32_is_stash_index(u32));
		Self(u32)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyStoreIndex(u32);

impl KeyStoreIndex {
	pub fn to_u32(&self) -> u32 { self.0 }
}

impl From<&KeyField> for KeyStoreIndex {
	fn from(value: &KeyField) -> Self {
		let value = key_field_to_store_index(value.0);
		Self(value)
	}
}

impl From<u32> for KeyStoreIndex {
	fn from(value: u32) -> Self { Self(value) }
}

pub trait KeyStore<K: Key>: ReadKey<K> {
	fn write_key(&mut self, key: &K) -> io::Result<KeyStoreIndex>;
}

pub trait ReadKey<K: Key> {
	fn read_key(&self, index: KeyStoreIndex) -> io::Result<K>;
}

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
