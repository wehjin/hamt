use crate::kv_forest::KvForest;
use crate::kv_forest::tests::prepare_kv_store_test_dir;

#[test]
fn persist_empty() {
	let path = prepare_kv_store_test_dir("persist-empty");
	let index = {
		let mut forest = KvForest::<u32>::open(&path.join("forest")).expect("open or create");
		forest.add_root().expect("index")
	};
	let forest = KvForest::<u32>::open(&path.join("forest")).expect("open or create");
	let trie = forest.trie(index).expect("trie at index");
	assert_eq!(0, trie.size());
}

#[test]
fn persist_thousand_internal_71() {
	let path = prepare_kv_store_test_dir("persist-thousand-interval-71");
	let index = {
		let mut forest = KvForest::<u32>::open(&path.join("forest")).expect("open or create");
		let mut index = forest.add_root().expect("add-root");
		for i in 0..1000 {
			index = forest.push(index, i * 71, i + 1).expect("push");
		}
		index
	};
	let forest = KvForest::<u32>::open(&path.join("forest")).expect("open or create");
	let trie = forest.trie(index).expect("trie at index");
	assert_eq!(1000, trie.size());
	for i in 0..1000 {
		assert_eq!(Some(i + 1), forest.find(index, &(i * 71)));
	}
}