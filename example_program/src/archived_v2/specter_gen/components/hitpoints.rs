///////////////////////////////////////////////////////////////
// THIS IS GENERATED CODE AND SHOULD NOT BE MODIFIED BY HAND //
///////////////////////////////////////////////////////////////

use specs::prelude::*;

use crate::specter_gen::data_types::number::NumberDataType;

pub struct HitpointsComponent {
	pub value: NumberDataType,
	pub max: NumberDataType,
}

impl Component for HitpointsComponent {
	type Storage = VecStorage<Self>;
}