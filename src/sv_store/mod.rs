use std::{fs, io};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use crate::kv_store::{KvForest, RootIndex};

pub struct StringValueStore {
	store_path: PathBuf,
	kv_forest: KvForest,
	int_keys: HashMap<String, u32>,
}

impl StringValueStore {
	pub fn create(store_path: impl AsRef<Path>) -> io::Result<()> {
		let path = store_path.as_ref();
		fs::create_dir(path)?;
		KvForest::create(Self::forest_path(path))?;
		{
			// Touch the file to set the mode.
			OpenOptions::new().write(true).mode(0o600).create_new(true).open(&Self::int_keys_path(path))?;
		}
		Self::write_int_keys(&path, &HashMap::new())?;
		Ok(())
	}
	fn write_int_keys(store_path: &Path, int_keys: &HashMap<String, u32>) -> io::Result<()> {
		let path = Self::int_keys_path(store_path);
		let json = serde_json::to_string(int_keys)?;
		fs::write(&path, &json)?;
		Ok(())
	}
	fn int_keys_path(path: &Path) -> PathBuf {
		let strings_path = path.join("strings");
		strings_path
	}
	fn forest_path(path: &Path) -> PathBuf {
		let forest_path = path.join("forest");
		forest_path
	}
	pub fn open(store_path: impl AsRef<Path>) -> io::Result<Self> {
		let store_path = store_path.as_ref();
		if !store_path.is_dir() {
			return Err(io::Error::from(io::ErrorKind::NotFound));
		}
		let kv_forest = KvForest::open(Self::forest_path(store_path))?;
		let int_keys = Self::read_int_keys(store_path)?;
		let store = Self { store_path: store_path.to_path_buf(), kv_forest, int_keys };
		Ok(store)
	}
	fn read_int_keys(store_path: &Path) -> io::Result<HashMap<String, u32>> {
		let path = Self::int_keys_path(store_path);
		let json = fs::read(&path)?;
		let int_keys = serde_json::from_slice::<HashMap<String, u32>>(json.as_slice())?;
		Ok(int_keys)
	}
	pub fn path(&self) -> &Path { &self.store_path }
	pub fn add_root(&mut self) -> io::Result<RootIndex> { self.kv_forest.add_root() }
	pub fn push(&mut self, root: RootIndex, key: impl AsRef<str>, value: u32) -> io::Result<RootIndex> {
		let key = key.as_ref().to_string();
		let int_key = if let Some(int_key) = self.int_keys.get(&key) {
			*int_key
		} else {
			let int_key = self.int_keys.len() as u32;
			self.int_keys.insert(key, int_key);
			Self::write_int_keys(&self.store_path, &self.int_keys)?;
			int_key
		};
		let new_root = self.kv_forest.push(root, int_key, value)?;
		Ok(new_root)
	}
	pub fn find(&self, root: RootIndex, key: impl AsRef<str>) -> Option<u32> {
		let key = &key.as_ref().to_string();
		match self.int_keys.get(key) {
			None => None,
			Some(int_key) => {
				let trie = self.kv_forest.trie(root).ok();
				let found = trie.map(|trie| trie.find(int_key).cloned()).flatten();
				found
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::sv_store::StringValueStore;
	use crate::tests::ready_test_dir;

	#[test]
	pub fn basic() {
		let test_dir = dbg!(ready_test_dir("sv-basic"));
		let store_dir = test_dir.join("sv");
		StringValueStore::create(&store_dir).expect("create store");
		let mut store = StringValueStore::open(&store_dir).expect("open store");
		assert_eq!(&store_dir, store.path());
		let root = store.add_root().expect("adds root to store");
		let root = store.push(root, "lot", 10).expect("store value for key");
		let root = store.push(root, "size", 10).expect("store value for key");
		let found = (store.find(root, "lot"), store.find(root, "size"));
		assert_eq!((Some(10), Some(10)), found);
	}
}
