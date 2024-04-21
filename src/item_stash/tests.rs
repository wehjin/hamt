use crate::item_stash::stash::ItemStash;
use crate::item_stash::tests::tools::named_test_dir;

#[test]
fn basic() {
	let test_dir = named_test_dir("basic");
	ItemStash::create(&test_dir).expect("create item-stash");
	let mut stash = ItemStash::open(&test_dir).expect("open item-stash");
	dbg!(stash);
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