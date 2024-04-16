pub trait HamtKey: Clone {
	fn key_byte(&self, offset: usize) -> u8;
}
