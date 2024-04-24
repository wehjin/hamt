use crate::kv_store::KvForest;
use crate::kv_store::tests::prepare_kv_store_test_dir;

#[test]
fn persist_empty() {
	let path = prepare_kv_store_test_dir("persist-empty");
	let index = {
		let mut forest = KvForest::open_or_create(&path).expect("open or create");
		forest.add_root().expect("index")
	};
	let forest = KvForest::open_or_create(&path).expect("open or create");
	let trie = forest.trie(index).expect("trie at index");
	assert_eq!(0, trie.size());
}

#[test]
fn persist_one() {
	let path = prepare_kv_store_test_dir("persist-one");
	let index = {
		let mut forest = KvForest::open_or_create(&path).expect("open or create");
		let index = forest.add_root().expect("add-root");
		forest.push(index, 3, 3).expect("push")
	};
	let forest = KvForest::open_or_create(&path).expect("open or create");
	let trie = forest.trie(index).expect("trie at index");
	assert_eq!(1, trie.size());
	assert_eq!(Some(3), trie.find(&3).cloned());
}