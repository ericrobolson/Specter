//THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND
use specs::prelude::*;
use crate::specter_gen::types::*;
pub struct VelocityComponent {
	pub value: Vec2,
}

impl Component for VelocityComponent {
	type Storage = VecStorage<Self>;
}