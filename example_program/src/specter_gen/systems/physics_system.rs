/////////////////////////////////////////////////////////////////
// THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND //
/////////////////////////////////////////////////////////////////
use specs::prelude::*;
use crate::specter_gen::types::*;

use crate::specter_gen::components::position_component::PositionComponent;
use crate::specter_gen::components::velocity_component::VelocityComponent;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
	type SystemData = (
		WriteStorage<'a, PositionComponent>,
		ReadStorage<'a, VelocityComponent>,
	);

	fn run(&mut self, (mut positions, velocitys): Self::SystemData) { 

 	}
 }