use specs::prelude::*;

pub struct VelocityComponent {
	pub value: vec2,
}

impl Component for VelocityComponent {
	type Storage = VecStorage<Self>;
}