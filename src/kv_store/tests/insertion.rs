use super::*;

#[test]
fn insert_two_keys_with_same_prefix_and_depth_and_two_more_at_lower_depth_finds_all_four_values() {
	let path = prepare_kv_store_test_dir("insert-a");
	let mut forest = KvForest::open_or_create(path).expect("open or create");
	let index = forest.new_trie().expect("index");
	let _trie = forest.push_trie(index, TestKey(0b000000000100000), 1);
	let _trie = forest.push_trie(index, TestKey(0b000000001000000), 2);
	let _trie = forest.push_trie(index, TestKey(0b000000001000001), 3);
	forest.push_trie(index, TestKey(0b000000001000010), 4).expect("insert");
	assert_eq!(4, forest[index].size());
	assert_eq!(Some(&1), forest[index].find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), forest[index].find(&TestKey(0b000000001000000)));
	assert_eq!(Some(&3), forest[index].find(&TestKey(0b000000001000001)));
	assert_eq!(Some(&4), forest[index].find(&TestKey(0b000000001000010)));
	assert_eq!(None, forest[index].find(&TestKey(0)));
}

#[test]
fn insert_two_keys_with_same_prefix_and_depth_and_third_at_lower_depth_finds_all_three_values() {
	let path = prepare_kv_store_test_dir("insert-b");
	let mut forest = KvForest::open_or_create(path).expect("open or create");
	let index = forest.new_trie().expect("index");
	let _trie = forest.push_trie(index, TestKey(0b000000000100000), 1);
	let _trie = forest.push_trie(index, TestKey(0b000000001000000), 2);
	forest.push_trie(index, TestKey(0b000000001000001), 3).expect("insert");
	assert_eq!(3, forest[index].size());
	assert_eq!(Some(&1), forest[index].find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), forest[index].find(&TestKey(0b000000001000000)));
	assert_eq!(Some(&3), forest[index].find(&TestKey(0b000000001000001)));
	assert_eq!(None, forest[index].find(&TestKey(0)));
}

#[test]
fn insert_three_keys_with_same_prefix_and_depth_finds_all_three_values() {
	let path = prepare_kv_store_test_dir("insert-c");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.new_trie().expect("index");
	let _trie = store.push_trie(index, TestKey(0b000000000100000), 1);
	let _trie = store.push_trie(index, TestKey(0b000000001000000), 2);
	store.push_trie(index, TestKey(0b000000001100000), 3).expect("insert");
	assert_eq!(3, store[index].size());
	assert_eq!(Some(&1), store[index].find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), store[index].find(&TestKey(0b000000001000000)));
	assert_eq!(Some(&3), store[index].find(&TestKey(0b000000001100000)));
	assert_eq!(None, store[index].find(&TestKey(0)));
}

#[test]
fn insert_two_keys_with_same_prefix_and_depth_finds_both_values() {
	let path = prepare_kv_store_test_dir("insert-");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.new_trie().expect("index");
	let _trie = store.push_trie(index, TestKey(0b000000000100000), 1);
	store.push_trie(index, TestKey(0b000000001000000), 2).expect("insert");
	assert_eq!(2, store[index].size());
	assert_eq!(Some(&1), store[index].find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), store[index].find(&TestKey(0b000000001000000)));
	assert_eq!(None, store[index].find(&TestKey(0)));
}

#[test]
fn insert_two_keys_on_different_paths_finds_both_values() {
	let path = prepare_kv_store_test_dir("insert-e");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.new_trie().expect("index");
	let _trie = store.push_trie(index, TestKey(0b000000000100000), 1);
	store.push_trie(index, TestKey(0b000010000100000), 33).expect("insert");
	assert_eq!(2, store[index].size());
	assert_eq!(Some(&1), store[index].find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&33), store[index].find(&TestKey(0b000010000100000)));
	assert_eq!(None, store[index].find(&TestKey(0)));
}

#[test]
fn insert_same_key_finds_last_value() {
	let path = prepare_kv_store_test_dir("insert-f");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.new_trie().expect("index");
	let _trie = store.push_trie(index, TestKey(0b000010000000000), 1);
	store.push_trie(index, TestKey(0b000010000000000), 2).expect("insert");
	assert_eq!(1, store[index].size());
	assert_eq!(Some(&2), store[index].find(&TestKey(0b000010000000000)));
	assert_eq!(None, store[index].find(&TestKey(0)));
}

#[test]
fn insert_value_finds_value() {
	let path = prepare_kv_store_test_dir("insert-g");
	let mut store = KvForest::open_or_create(path).expect("open or create");
	let index = store.new_trie().expect("index");
	store.push_trie(index, TestKey(0b000010000000000), 1).expect("insert");
	assert_eq!(1, store[index].size());
	assert_eq!(Some(&1), store[index].find(&TestKey(0b000010000000000)));
	assert_eq!(None, store[index].find(&TestKey(0)));
}
