use std::io;

use index::KeyStoreIndex;

#[cfg(test)]
mod tests {
	use crate::key_store::{KeyStore, ReadKey};
	use crate::key_store::u32::U32KeyStore;

	#[test]
	fn basic() {
		let store = &mut U32KeyStore;
		let write_key = 137u32;
		let key_store_index = store.write_key(&write_key).expect("write_key");
		let read_key = store.read_key(key_store_index).expect("read_key");
		assert_eq!(write_key, read_key);
	}
}

pub mod field;
pub mod index;
pub mod string;
pub mod u32;

pub trait Key: Eq {
	fn to_shard(&self, depth: usize) -> u8;
}

pub trait KeyStore<K: Key>: ReadKey<K> {
	fn write_key(&mut self, key: &K) -> io::Result<KeyStoreIndex>;
}

impl<K: Key, T: KeyStore<K>> KeyStore<K> for Box<T> {
	fn write_key(&mut self, key: &K) -> io::Result<KeyStoreIndex> {
		self.as_mut().write_key(key)
	}
}

pub trait ReadKey<K: Key> {
	fn read_key(&self, index: KeyStoreIndex) -> io::Result<K>;
}

impl<K: Key, T: ReadKey<K>> ReadKey<K> for Box<T> {
	fn read_key(&self, index: KeyStoreIndex) -> io::Result<K> {
		self.as_ref().read_key(index)
	}
}
