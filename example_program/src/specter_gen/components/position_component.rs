//THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND
use specs::prelude::*;
use crate::specter_gen::types::*;
pub struct PositionComponent {
	pub value: Vec2,
}

impl Component for PositionComponent {
	type Storage = VecStorage<Self>;
}