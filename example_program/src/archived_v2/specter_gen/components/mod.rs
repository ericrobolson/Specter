///////////////////////////////////////////////////////////////
// THIS IS GENERATED CODE AND SHOULD NOT BE MODIFIED BY HAND //
///////////////////////////////////////////////////////////////

pub mod cleanup;
pub mod hitpoints;
pub mod position;
pub mod velocity;

use crate::specter_gen::components::position::PositionComponent;
use crate::specter_gen::components::velocity::VelocityComponent;
use crate::specter_gen::components::hitpoints::HitpointsComponent;
use crate::specter_gen::components::cleanup::CleanupComponent;

use specs::prelude::*;


pub fn world_linker(world: &mut specs::World) {
	world.register::<PositionComponent>();
	world.register::<VelocityComponent>();
	world.register::<HitpointsComponent>();
	world.register::<CleanupComponent>();
}