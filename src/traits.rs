pub trait HamtKey: Clone + PartialEq {
	fn key_byte(&self, offset: usize) -> u8;
}
