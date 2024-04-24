use std::ops::Add;

pub(crate) const ELEMENT_BYTES: usize = 8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ElementStoreIndex(pub u32);

impl ElementStoreIndex {
	pub fn to_file_position(&self) -> u64 {
		(self.0 as usize * ELEMENT_BYTES) as u64
	}
	pub fn from_file_position(file_position: u64) -> Self {
		let index = (file_position as usize / ELEMENT_BYTES) as u32;
		Self(index)
	}
}

impl Add<isize> for ElementStoreIndex {
	type Output = ElementStoreIndex;

	fn add(self, rhs: isize) -> Self::Output {
		let next = rhs + self.0 as isize;
		Self(next as u32)
	}
}
