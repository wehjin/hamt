pub(crate) const ELEMENT_BYTES: usize = 8;

#[derive(Debug, Copy, Clone)]
pub struct ElementStoreIndex(pub u32);

impl ElementStoreIndex {
	pub fn to_file_position(&self, offset: usize) -> u64 {
		(self.0 as usize + ELEMENT_BYTES * offset) as u64
	}
	pub fn from_file_position(file_position: u64) -> Self {
		Self((file_position as usize / ELEMENT_BYTES) as u32)
	}
}
