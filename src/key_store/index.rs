use crate::key_store::field::KeyField;
use crate::key_store::u32;
use crate::trie::key_field_to_store_index;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyStoreIndex(pub(crate) u32);

impl KeyStoreIndex {
	pub fn to_u32(&self) -> u32 { self.to_file_pos() as u32 }
	pub fn to_file_pos(&self) -> u64 { self.0 as u64 }
}

impl From<&KeyField> for KeyStoreIndex {
	fn from(value: &KeyField) -> Self {
		let value = key_field_to_store_index(value.0);
		Self(value)
	}
}

impl From<u32> for KeyStoreIndex {
	fn from(value: u32) -> Self { Self(value) }
}

