use std::io;

use crate::trie::u32_key_byte;

#[cfg(test)]
mod tests {
	use crate::key_store::{KeyStore, ReadKey, U32KeyStore};

	#[test]
	fn basic() {
		let store = &mut U32KeyStore;
		let key = 137u32;
		let pos = store.write_key(&key).expect("write_key");
		let reader = store.to_read_key();
		let read_key = reader.read_key(pos).expect("read_key");
		assert_eq!(key, read_key);
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
pub struct KeyIndex(u32);

impl KeyIndex {
	pub fn to_u32(&self) -> u32 { self.0 }
}

impl From<u32> for KeyIndex {
	fn from(value: u32) -> Self { Self(value) }
}

pub trait KeyStore<K: Key> {
	fn write_key(&mut self, key: &K) -> io::Result<KeyIndex>;
	fn to_read_key(&self) -> impl ReadKey<K>;
}

pub trait ReadKey<K: Key> {
	fn read_key(&self, index: KeyIndex) -> io::Result<K>;
}

pub struct U32KeyStore;

impl KeyStore<u32> for U32KeyStore {
	fn write_key(&mut self, key: &u32) -> io::Result<KeyIndex> {
		Ok(KeyIndex::from(*key))
	}
	fn to_read_key(&self) -> impl ReadKey<u32> { U32KeyRead }
}

pub struct U32KeyRead;

impl ReadKey<u32> for U32KeyRead {
	fn read_key(&self, index: KeyIndex) -> io::Result<u32> {
		Ok(index.to_u32())
	}
}