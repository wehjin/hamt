use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::fs::{FileExt, OpenOptionsExt};
use std::path::Path;

use crate::key_store::{Key, KeyStore, ReadKey};
use crate::key_store::index::KeyStoreIndex;

#[cfg(test)]
mod tests {
	use crate::key_store::{KeyStore, ReadKey};
	use crate::key_store::string::StringKeyStore;
	use crate::tests::ready_test_dir;

	#[test]
	fn ks_basic() {
		let test_dir = ready_test_dir("string-key-store");
		let store_dir = test_dir.join("store");
		let mut store = StringKeyStore::open(store_dir).expect("open");
		let key = "Hello!".to_string();
		let index = store.write_key(&key).expect("write_key");
		let read_key = store.read_key(index).expect("read_key");
		assert_eq!(key, read_key);
	}
}

impl Key for String {
	fn to_shard(&self, depth: usize) -> u8 {
		let full_byte = self.as_bytes()[depth / 2];
		match (depth % 2) == 0 {
			true => full_byte >> 4,
			false => full_byte & 0x0f,
		}
	}
}


pub struct StringKeyStore {
	file: File,
}

impl StringKeyStore {
	pub fn open(store_path: impl AsRef<Path>) -> io::Result<Self> {
		let path = store_path.as_ref();
		let file = if !path.exists() {
			OpenOptions::new().mode(0o600).create(true).read(true).append(true).open(path)?
		} else {
			OpenOptions::new().read(true).append(true).open(path)?
		};
		Ok(Self { file })
	}
}

impl ReadKey<String> for StringKeyStore {
	fn read_key(&self, index: KeyStoreIndex) -> io::Result<String> {
		let size = {
			let mut size_bytes = [0u8; 2];
			self.file.read_exact_at(&mut size_bytes, index.to_file_pos())?;
			decode_size(size_bytes)
		};
		let string = {
			let mut buffer = vec![0u8; size];
			self.file.read_exact_at(&mut buffer, index.to_file_pos() + 2)?;
			String::from_utf8(buffer).expect("utf8 in buffer")
		};
		Ok(string)
	}
}

fn decode_size(bytes: [u8; 2]) -> usize {
	u16::from_be_bytes(bytes) as usize
}

impl KeyStore<String> for StringKeyStore {
	fn write_key(&mut self, key: &String) -> io::Result<KeyStoreIndex> {
		let pos = self.file.seek(SeekFrom::End(0))?;
		let bytes = key.as_bytes();
		let size_bytes = encode_size(bytes.len());
		self.file.write_all(&size_bytes)?;
		self.file.write_all(bytes)?;
		let index = KeyStoreIndex(pos as u32);
		Ok(index)
	}
}

fn encode_size(size: usize) -> [u8; 2] {
	(size as u16).to_be_bytes()
}
