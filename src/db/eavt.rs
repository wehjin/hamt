use std::{fs, io};
use std::path::Path;

use crate::kv_forest::{KvForest, RootIndex};

#[cfg(test)]
mod tests {
	use crate::db::eavt::Eavt;
	use crate::tests::ready_test_dir;

	#[test]
	fn basic() -> anyhow::Result<()> {
		let forest_dir = ready_test_dir("db-eavt-basic").join("forest");
		let mut forest = Eavt::open(&forest_dir)?;
		let root = forest.new_root()?;
		let root = forest.push(root, 0, 0, "size".into(), 42)?;
		let found = forest.find_t(root, &0, &0, &"size".into());
		assert_eq!(Some(42), found);
		Ok(())
	}
}

pub struct Eavt {
	eavt: KvForest<u32>,
	avt: KvForest<u32>,
	vt: KvForest<String>,
}

impl Eavt {
	pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
		let path = path.as_ref();
		if !path.is_dir() {
			try_create_dir(path)?;
		}
		let vt = KvForest::<String>::open(&path.join("vt.forest"))?;
		let avt = KvForest::<u32>::open(&path.join("avt.forest"))?;
		let eavt = KvForest::<u32>::open(&path.join("eavt.forest"))?;
		Ok(Self { vt, avt, eavt })
	}
	pub fn new_root(&mut self) -> io::Result<RootIndex> { self.eavt.add_root() }

	pub fn find_t(&self, root_index: RootIndex, e: &u32, a: &u32, v: &String) -> Option<u32> {
		let eavt_root = root_index;
		let avt_root = match self.eavt.find(eavt_root, e) {
			None => return None,
			Some(found) => RootIndex::from(found)
		};
		let vt_root = match self.avt.find(avt_root, a) {
			None => return None,
			Some(found) => RootIndex::from(found)
		};
		let t = self.vt.find(vt_root, v);
		t
	}

	pub fn push(&mut self, root_index: RootIndex, e: u32, a: u32, v: String, t: u32) -> io::Result<RootIndex> {
		let eavt_root = root_index;
		let avt_root = match self.eavt.find(eavt_root, &e) {
			None => self.avt.add_root()?,
			Some(found) => RootIndex::from(found),
		};
		let vt_root = match self.avt.find(avt_root, &a) {
			None => self.vt.add_root()?,
			Some(found) => RootIndex::from(found)
		};
		let no_change = self.vt.find(vt_root, &v).map(|old| old == t).unwrap_or(false);
		let output = match no_change {
			true => root_index,
			false => {
				let new_vt_root = self.vt.push(vt_root, v, t)?;
				let new_avt_root = self.avt.push(avt_root, a, new_vt_root.to_u32())?;
				let new_eavt_root = self.eavt.push(eavt_root, e, new_avt_root.to_u32())?;
				new_eavt_root
			}
		};
		Ok(output)
	}
}

fn try_create_dir(path: impl AsRef<Path>) -> io::Result<()> {
	if path.as_ref().exists() {
		return Err(io::Error::from(io::ErrorKind::AlreadyExists));
	}
	fs::create_dir(path)?;
	Ok(())
}
