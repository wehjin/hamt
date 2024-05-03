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
		KvForest::<u32>::open(&forest_dir).expect("open");
	}

	#[test]
	fn basic_string() -> anyhow::Result<()> {
		let test_dir = prepare_kv_store_test_dir("basic-string");
		let forest_dir = test_dir.join("forest_dir");
		let mut forest = KvForest::<String>::open(&forest_dir).expect("open");
		let index = forest.add_root()?;
		let key = "Hey".to_string();
		let index = forest.push(index, key.clone(), 10)?;
		let read = forest.find(index, &key);
		assert_eq!(Some(10), read);
		Ok(())
	}
}

fn prepare_kv_store_test_dir(name: &str) -> PathBuf {
	ready_test_dir(&format!("kv_store-{}", name))
}
