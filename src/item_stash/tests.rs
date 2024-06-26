use crate::item_stash::element::ElementStoreIndex;
use crate::item_stash::stash::ItemStash;
use crate::item_stash::tests::tools::named_test_dir;

#[test]
fn basic() {
	let test_dir = named_test_dir("item-stash-basic");
	ItemStash::create(&test_dir).expect("create item-stash");
	{
		let mut stash = ItemStash::open(&test_dir).expect("open item-stash");
		assert_eq!(0, stash.len());
		let index = stash.append([[1, 1], [2, 2], ]).expect("append");
		assert_eq!(2, stash.len());

		let read = stash.to_element_read().expect("read");
		let bytes = [
			read.read(index).expect(&format!("read 0 from {:?}", &test_dir)),
			read.read(index + 1).expect("read 1"),
		];
		assert_eq!([[0, 0, 0, 1, 0, 0, 0, 1], [0, 0, 0, 2, 0, 0, 0, 2]], bytes);
	}
	{
		let stash = ItemStash::open(&test_dir).expect("reopen item-stash");
		assert_eq!(2, stash.len());
		let position = ElementStoreIndex(0);
		let read = stash.to_element_read().expect("read");
		let bytes = [
			read.read(position).expect("read 0"),
			read.read(position + 1).expect("read 1"),
		];
		assert_eq!([[0, 0, 0, 1, 0, 0, 0, 1], [0, 0, 0, 2, 0, 0, 0, 2]], bytes);
	}
}

mod tools {
	use std::{env, fs};
	use std::path::PathBuf;

	pub fn named_test_dir(name: &str) -> PathBuf {
		let path = test_dir().join(name);
		if path.exists() {
			fs::remove_dir_all(&path).expect("remove test dir");
		}
		path
	}

	pub fn test_dir() -> PathBuf {
		let path = env::temp_dir().join("item_stash");
		if !path.exists() {
			fs::create_dir_all(&path).expect("create dir for item_stash tests");
		}
		path
	}
}