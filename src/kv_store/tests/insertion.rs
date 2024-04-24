use super::*;

#[test]
fn insert_two_keys_with_same_prefix_and_depth_and_two_more_at_lower_depth_finds_all_four_values() {
	let path = prepare_kv_store_test_dir("insert-a");
	let mut forest = KvForest::open_or_create(path).expect("open or create");
	let index = forest.add_root().expect("index");
	let index = forest.push(index, 0b000000000100000, 1).expect("push");
	let index = forest.push(index, 0b000000001000000, 2).expect("push");
	let index = forest.push(index, 0b000000001000001, 3).expect("push");
	let index = forest.push(index, 0b000000001000010, 4).expect("push");
	let trie = forest.trie(index).expect("trie at index");
	assert_eq!(4, trie.size());
	assert_eq!(Some(&1), trie.find(&0b000000000100000));
	assert_eq!(Some(&2), trie.find(&0b000000001000000));
	assert_eq!(Some(&3), trie.find(&0b000000001000001));
	assert_eq!(Some(&4), trie.find(&0b000000001000010));
	assert_eq!(None, trie.find(&0));
}

#[test]
fn insert_two_keys_with_same_prefix_and_depth_and_third_at_lower_depth_finds_all_three_values() {
	let path = prepare_kv_store_test_dir("insert-b");
	let mut forest = KvForest::open_or_create(path).expect("open or create");
	let index = forest.add_root().expect("index");
	let index = forest.push(index, 0b000000000100000, 1).expect("push");
	let index = forest.push(index, 0b000000001000000, 2).expect("push");
	let index = forest.push(index, 0b000000001000001, 3).expect("push");
	let trie = forest.trie(index).expect("trie");
	assert_eq!(3, trie.size());
	assert_eq!(Some(&1), trie.find(&0b000000000100000));
	assert_eq!(Some(&2), trie.find(&0b000000001000000));
	assert_eq!(Some(&3), trie.find(&0b000000001000001));
	assert_eq!(None, trie.find(&0));
}

#[test]
fn insert_three_keys_with_same_prefix_and_depth_finds_all_three_values() {
	let path = prepare_kv_store_test_dir("insert-c");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.add_root().expect("index");
	let index = store.push(index, 0b000000000100000, 1).expect("push");
	let index = store.push(index, 0b000000001000000, 2).expect("push");
	let index = store.push(index, 0b000000001100000, 3).expect("push");
	let trie = store.trie(index).expect("trie");
	assert_eq!(3, trie.size());
	assert_eq!(Some(&1), trie.find(&0b000000000100000));
	assert_eq!(Some(&2), trie.find(&0b000000001000000));
	assert_eq!(Some(&3), trie.find(&0b000000001100000));
	assert_eq!(None, trie.find(&0));
}

#[test]
fn insert_two_keys_with_same_prefix_and_depth_finds_both_values() {
	let path = prepare_kv_store_test_dir("insert-");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.add_root().expect("index");
	let index = store.push(index, 0b000000000100000, 1).expect("push");
	let index = store.push(index, 0b000000001000000, 2).expect("push");
	let trie = store.trie(index).expect("trie");
	assert_eq!(2, trie.size());
	assert_eq!(Some(&1), trie.find(&0b000000000100000));
	assert_eq!(Some(&2), trie.find(&0b000000001000000));
	assert_eq!(None, trie.find(&0));
}

#[test]
fn insert_two_keys_on_different_paths_finds_both_values() {
	let path = prepare_kv_store_test_dir("insert-e");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.add_root().expect("index");
	let index = store.push(index, 0b000000000100000, 1).expect("push");
	let index = store.push(index, 0b000010000100000, 33).expect("push");
	let trie = store.trie(index).expect("trie");
	assert_eq!(2, trie.size());
	assert_eq!(Some(&1), trie.find(&0b000000000100000));
	assert_eq!(Some(&33), trie.find(&0b000010000100000));
	assert_eq!(None, trie.find(&0));
}

#[test]
fn insert_same_key_finds_last_value() {
	let path = prepare_kv_store_test_dir("insert-f");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.add_root().expect("index");
	let index = store.push(index, 0b000010000000000, 1).expect("push");
	let index = store.push(index, 0b000010000000000, 2).expect("push");
	let trie = store.trie(index).expect("trie");
	assert_eq!(1, trie.size());
	assert_eq!(Some(&2), trie.find(&0b000010000000000));
	assert_eq!(None, trie.find(&0));
}

#[test]
fn insert_value_finds_value() {
	let path = prepare_kv_store_test_dir("insert-g");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.add_root().expect("index");
	let index = store.push(index, 0b000010000000000, 1).expect("push");
	let trie = store.trie(index).expect("trie");
	assert_eq!(1, trie.size());
	assert_eq!(Some(&1), trie.find(&0b000010000000000));
	assert_eq!(None, trie.find(&0));
}
