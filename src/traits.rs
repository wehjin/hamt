pub trait HamtKey {
	fn key_byte(&self, offset: usize) -> u8;
}
