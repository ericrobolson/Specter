//THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND
use specs::prelude::*;
use crate::specter_gen::types::*;

pub mod position_component;
use position_component::PositionComponent;

pub mod velocity_component;
use velocity_component::VelocityComponent;

pub fn link_components(world: &mut specs::World) {
	world.register::<PositionComponent>();
	world.register::<VelocityComponent>();
}