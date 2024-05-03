use std::path::PathBuf;

use crate::tests::ready_test_dir;

use super::*;

mod insertion;
mod persistence;

mod basic {
	use crate::kv_forest::KvForest;
	use crate::kv_forest::tests::prepare_kv_store_test_dir;

	#[test]
	fn basic() {
		let test_dir = prepare_kv_store_test_dir("basic");
		let forest_dir = test_dir.join("forest_dir");
		KvForest::<u32>::open_or_create(&forest_dir).expect("open");
	}
}

fn prepare_kv_store_test_dir(name: &str) -> PathBuf {
	ready_test_dir(&format!("kv_store-{}", name))
}
