use crate::array_map::ElementMap;

#[cfg(test)]
mod tests {
	use crate::traits::HamtKey;

	struct Key(u32);

	impl HamtKey for Key {
		fn key_byte(&self, offset: usize) -> u8 {
			const SHIFT: [u32; 7] = [30, 25, 20, 15, 10, 5, 0];
			(self.0 >> SHIFT[offset % 7]) as u8
		}
	}
}

pub mod array_map;
pub mod datom;
pub mod item_store;
pub mod traits;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Trie<K, V> {
	map: ElementMap,
	elements: ElementList<K, V>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ElementList<K, V>(pub Vec<Element<K, V>>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element<K, V> {
	KeyValue(K, V),
	SubTrie(Trie<K, V>),
}
