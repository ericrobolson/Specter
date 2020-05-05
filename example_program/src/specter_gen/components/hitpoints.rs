///////////////////////////////////////////////////////////////
// THIS IS GENERATED CODE AND SHOULD NOT BE MODIFIED BY HAND //
///////////////////////////////////////////////////////////////

use specs::prelude::*;

pub struct HitpointsComponent {
	pub value: number,
	pub max: number,
}

impl Component for HitpointsComponent {
	type Storage = VecStorage<Self>;
}