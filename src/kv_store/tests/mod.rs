use std::env;
use std::path::PathBuf;

use super::*;

mod insertion;
mod persistence;

mod basic {
	use crate::kv_store::KvForest;
	use crate::kv_store::tests::prepare_kv_store_test_dir;

	#[test]
	fn basic() {
		let test_dir = prepare_kv_store_test_dir("basic");
		KvForest::create(&test_dir).expect("create kv-store");
		KvForest::open(&test_dir).expect("open");
	}
}

fn prepare_kv_store_test_dir(name: &str) -> PathBuf {
	let test_dir = env::temp_dir().join("kv_store").join(name);
	if test_dir.exists() && test_dir.is_dir() {
		fs::remove_dir_all(&test_dir).expect("remove test dir");
	}
	test_dir
}
