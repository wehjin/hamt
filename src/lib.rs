pub mod attribute;
pub mod datom;
pub mod item_stash;
pub mod kv_store;
pub mod trie;


pub mod db {
	use std::{fs, io};
	use std::path::{Path, PathBuf};

	use crate::attribute::Attribute;

	pub struct Db {
		path: PathBuf,
	}

	impl Db {
		pub fn create(db_dir: impl AsRef<Path>, attributes: impl AsRef<[Attribute]>) -> io::Result<()> {
			fs::create_dir(db_dir.as_ref())?;
			Ok(())
		}
		pub fn open(db_dir: impl AsRef<Path>) -> io::Result<Self> {
			let path = db_dir.as_ref().to_path_buf();
			if !path.exists() || !path.is_dir() {
				return Err(io::Error::from(io::ErrorKind::NotFound));
			}
			Ok(Self { path })
		}
		pub fn path(&self) -> &Path { &self.path }
	}

	#[cfg(test)]
	mod tests {
		use std::{env, fs};
		use std::path::PathBuf;

		use crate::attribute::Attribute;
		use crate::db::Db;

		#[test]
		fn basic() {
			let db_dir = dbg!(ready_test_dir("hamt-db-basic").join("db"));
			Db::create(
				&db_dir,
				&[Attribute("lot", "size")],
			).unwrap();
			let db = Db::open(&db_dir).expect("Open succeeds");
			assert_eq!(&db_dir, db.path());
		}

		fn ready_test_dir(name: &str) -> PathBuf {
			let test_dir = env::temp_dir().join(name);
			if test_dir.exists() {
				fs::remove_dir_all(&test_dir).expect("remove test dir");
			}
			fs::create_dir_all(&test_dir).expect("create test dir");
			test_dir
		}
	}
}