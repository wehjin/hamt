use std::{fs, io};
use std::path::{Path, PathBuf};

use attribute::Attribute;

pub mod attribute;

pub struct Db {
	path: PathBuf,
}

impl Db {
	pub fn create(db_dir: impl AsRef<Path>, _attributes: impl AsRef<[Attribute]>) -> io::Result<()> {
		fs::create_dir(db_dir.as_ref())?;

		const _SPACE_ENTITY: u32 = 10;
		Ok(())
	}
	pub fn open(db_dir: impl AsRef<Path>) -> io::Result<Self> {
		let path = db_dir.as_ref().to_path_buf();
		if !path.is_dir() {
			return Err(io::Error::from(io::ErrorKind::NotFound));
		}
		Ok(Self { path })
	}
	pub fn path(&self) -> &Path { &self.path }
}

#[cfg(test)]
mod tests {
	use crate::db::attribute::Attribute;
	use crate::db::Db;
	use crate::tests;

	#[test]
	fn basic() {
		let db_dir = tests::ready_test_dir("db-basic").join("db");
		Db::create(&db_dir, &[Attribute("lot", "size")]).unwrap();
		let db = Db::open(&db_dir).expect("Open succeeds");
		assert_eq!(&db_dir, db.path());
	}
}
