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

pub trait KeyStore<K: Key> {
	fn write_key(&mut self, key: &K) -> io::Result<u32>;
	fn to_read_key(&self) -> impl ReadKey<u32>;
}

pub trait ReadKey<K: Key> {
	fn read_key(&self, pos: u32) -> io::Result<K>;
}

pub struct U32KeyStore;

impl KeyStore<u32> for U32KeyStore {
	fn write_key(&mut self, key: &u32) -> io::Result<u32> {
		Ok(*key)
	}
	fn to_read_key(&self) -> impl ReadKey<u32> { U32KeyRead }
}

pub struct U32KeyRead;

impl ReadKey<u32> for U32KeyRead {
	fn read_key(&self, pos: u32) -> io::Result<u32> {
		Ok(pos)
	}
}