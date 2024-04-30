use std::{env, fs};
use std::path::PathBuf;

pub fn ready_test_dir(name: &str) -> PathBuf {
	let test_dir = env::temp_dir().join("hamt").join(name);
	if test_dir.exists() {
		fs::remove_dir_all(&test_dir).expect("remove test dir");
	}
	fs::create_dir_all(&test_dir).expect("create test dir");
	test_dir
}

