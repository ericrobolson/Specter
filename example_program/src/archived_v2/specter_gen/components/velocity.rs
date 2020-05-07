///////////////////////////////////////////////////////////////
// THIS IS GENERATED CODE AND SHOULD NOT BE MODIFIED BY HAND //
///////////////////////////////////////////////////////////////

use specs::prelude::*;

use crate::specter_gen::data_types::vec2::Vec2DataType;

pub struct VelocityComponent {
	pub value: Vec2DataType,
}

impl Component for VelocityComponent {
	type Storage = VecStorage<Self>;
}