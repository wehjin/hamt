use std::env;
use std::path::PathBuf;

use super::*;

mod insertion;
mod persistence;

mod basic {
	use crate::kv_store::KvForest;
	use crate::kv_store::tests::{prepare_kv_store_test_dir, TestKey};

	#[test]
	fn basic() {
		let test_dir = prepare_kv_store_test_dir("basic");
		KvForest::<TestKey, u32>::create(&test_dir).expect("create kv-store");
		let store = KvForest::<TestKey, u32>::open(&test_dir).expect("open");
	}
}

fn prepare_kv_store_test_dir(name: &str) -> PathBuf {
	let test_dir = env::temp_dir().join("kv_store").join(name);
	if test_dir.exists() {
		fs::remove_dir(&test_dir).expect("remove test dir");
	}
	test_dir
}


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
