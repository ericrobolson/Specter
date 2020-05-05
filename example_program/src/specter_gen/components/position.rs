use specs::prelude::*;

pub struct PositionComponent {
	pub value: vec2,
}

impl Component for PositionComponent {
	type Storage = VecStorage<Self>;
}