#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
	String(String)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EntityId(pub u32);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Effect { Add, Retract }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TxEvent(pub EntityId, pub Effect);
