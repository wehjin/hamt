use crate::key_store::u32;
use crate::key_store::index::KeyStoreIndex;
use crate::trie::{key_field_from_store_index, u32_from_bytes, u32_is_stash_index};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyField(pub(crate) u32);

impl KeyField {
	pub fn to_u32(&self) -> u32 { self.0 }
}

impl From<KeyStoreIndex> for KeyField {
	fn from(value: KeyStoreIndex) -> Self {
		Self(key_field_from_store_index(value.0))
	}
}

impl From<&[u8]> for KeyField {
	fn from(bytes: &[u8]) -> Self {
		let u32 = u32_from_bytes(bytes);
		debug_assert!(!u32_is_stash_index(u32));
		Self(u32)
	}
}

