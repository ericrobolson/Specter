use specs::prelude::*;

pub struct CleanupComponent {}

impl Component for CleanupComponent {
	type Storage = VecStorage<Self>;
}