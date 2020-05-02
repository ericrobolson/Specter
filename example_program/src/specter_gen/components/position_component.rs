//////////////////////////////////////////////////////////////////////////////////
// THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND - Specter v0.0.1 //
//////////////////////////////////////////////////////////////////////////////////
use specs::prelude::*;
use crate::specter_gen::types::*;
pub struct PositionComponent {
	pub value: Vec2,
}

impl Component for PositionComponent {
	type Storage = VecStorage<Self>;
}