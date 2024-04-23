use crate::kv_store::KvForest;
use crate::kv_store::tests::{prepare_kv_store_test_dir, TestKey};

#[test]
fn persist_empty() {
	let path = prepare_kv_store_test_dir("persist-empty");
	let index = {
		let mut forest = KvForest::<TestKey, u32>::open_or_create(&path).expect("open or create");
		forest.new_trie().expect("index")
	};
	let forest = KvForest::<TestKey, u32>::open_or_create(&path).expect("open or create");
	let size = forest[index].size();
	assert_eq!(0, size);
}