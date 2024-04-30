use std::{fs, io};
use std::path::{Path, PathBuf};

pub struct StringValueStore {
	path: PathBuf,
}

impl StringValueStore {
	pub fn create(store_path: impl AsRef<Path>) -> io::Result<()> {
		let path = store_path.as_ref();
		fs::create_dir(path)?;
		Ok(())
	}
	pub fn open(store_path: impl AsRef<Path>) -> io::Result<Self> {
		let path = store_path.as_ref().to_path_buf();
		if !path.is_dir() {
			return Err(io::Error::from(io::ErrorKind::NotFound));
		}
		Ok(Self { path })
	}
	pub fn path(&self) -> &Path { &self.path }
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
		let store = StringValueStore::open(&store_dir).expect("open store");
		assert_eq!(&store_dir, store.path());
	}
}
