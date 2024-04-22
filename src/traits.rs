use std::fmt::Debug;

pub trait HamtKey: Debug + Clone + PartialEq {
	fn key_byte(&self, offset: usize) -> u8;
}
