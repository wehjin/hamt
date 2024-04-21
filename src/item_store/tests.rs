use super::*;

#[test]
fn basic() {
	let mut store = ItemStore::new();
	let item = 42;
	let item_ref = store.push(item);
	assert_eq!(&item, item_ref.as_ref());
}
