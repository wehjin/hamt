use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct TestKey(u16);

impl HamtKey for TestKey {
	fn key_byte(&self, offset: usize) -> u8 {
		let shift_bits = match offset % 3 {
			0 => 10,
			1 => 5,
			2 => 0,
			_ => unreachable!("modulo 3")
		};
		((self.0 >> shift_bits) & 0b11111) as u8
	}
}

#[test]
fn insert_two_keys_with_same_prefix_and_depth_and_two_more_at_lower_depth_finds_all_four_values() {
	let mut store = KvStore::open();
	let _trie = store.insert_value(TestKey(0b000000000100000), 1);
	let _trie = store.insert_value(TestKey(0b000000001000000), 2);
	let _trie = store.insert_value(TestKey(0b000000001000001), 3);
	let trie = store.insert_value(TestKey(0b000000001000010), 4);
	assert_eq!(4, trie.size());
	assert_eq!(Some(&1), trie.find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), trie.find(&TestKey(0b000000001000000)));
	assert_eq!(Some(&3), trie.find(&TestKey(0b000000001000001)));
	assert_eq!(Some(&4), trie.find(&TestKey(0b000000001000010)));
	assert_eq!(None, trie.find(&TestKey(0)));
}

#[test]
fn insert_two_keys_with_same_prefix_and_depth_and_third_at_lower_depth_finds_all_three_values() {
	let mut store = KvStore::open();
	let _trie = store.insert_value(TestKey(0b000000000100000), 1);
	let _trie = store.insert_value(TestKey(0b000000001000000), 2);
	let trie = store.insert_value(TestKey(0b000000001000001), 3);
	assert_eq!(3, trie.size());
	assert_eq!(Some(&1), trie.find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), trie.find(&TestKey(0b000000001000000)));
	assert_eq!(Some(&3), trie.find(&TestKey(0b000000001000001)));
	assert_eq!(None, trie.find(&TestKey(0)));
}

#[test]
fn insert_three_keys_with_same_prefix_and_depth_finds_all_three_values() {
	let mut store = KvStore::open();
	let _trie = store.insert_value(TestKey(0b000000000100000), 1);
	let _trie = store.insert_value(TestKey(0b000000001000000), 2);
	let trie = store.insert_value(TestKey(0b000000001100000), 3);
	assert_eq!(3, trie.size());
	assert_eq!(Some(&1), trie.find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), trie.find(&TestKey(0b000000001000000)));
	assert_eq!(Some(&3), trie.find(&TestKey(0b000000001100000)));
	assert_eq!(None, trie.find(&TestKey(0)));
}

#[test]
fn insert_two_keys_with_same_prefix_and_depth_finds_both_values() {
	let mut store = KvStore::open();
	let _trie = store.insert_value(TestKey(0b000000000100000), 1);
	let trie = store.insert_value(TestKey(0b000000001000000), 2);
	assert_eq!(2, trie.size());
	assert_eq!(Some(&1), trie.find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&2), trie.find(&TestKey(0b000000001000000)));
	assert_eq!(None, trie.find(&TestKey(0)));
}

#[test]
fn insert_two_keys_on_different_paths_finds_both_values() {
	let mut store = KvStore::open();
	let _trie = store.insert_value(TestKey(0b000000000100000), 1);
	let trie = store.insert_value(TestKey(0b000010000100000), 33);
	assert_eq!(2, trie.size());
	assert_eq!(Some(&1), trie.find(&TestKey(0b000000000100000)));
	assert_eq!(Some(&33), trie.find(&TestKey(0b000010000100000)));
	assert_eq!(None, trie.find(&TestKey(0)));
}

#[test]
fn insert_same_key_finds_last_value() {
	let mut store = KvStore::open();
	let _trie = store.insert_value(TestKey(0b000010000000000), 1);
	let trie = store.insert_value(TestKey(0b000010000000000), 2);
	assert_eq!(1, trie.size());
	assert_eq!(Some(&2), trie.find(&TestKey(0b000010000000000)));
	assert_eq!(None, trie.find(&TestKey(0)));
}

#[test]
fn insert_value_finds_value() {
	let mut store = KvStore::open();
	let trie = store.insert_value(TestKey(0b000010000000000), 1);
	assert_eq!(1, trie.size());
	assert_eq!(Some(&1), trie.find(&TestKey(0b000010000000000)));
	assert_eq!(None, trie.find(&TestKey(0)));
}
