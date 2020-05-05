use specs::prelude::*;

use crate::specter_gen::components::position::PositionComponent;
use crate::specter_gen::components::velocity::VelocityComponent;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
	type SystemData = (
		WriteStorage<'a, PositionComponent>,
		ReadStorage<'a, VelocityComponent>,
	);
	
	fn run(&mut self, (mut positions, velocities): Self::SystemData) {
	}
}