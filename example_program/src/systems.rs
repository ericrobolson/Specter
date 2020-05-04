pub struct physics;

impl<'a> System<'a> for physics {
	type SystemData = (WriteStorage<'a, position>, ReadStorage<'a, velocity>);

	fn run(&mut self, (mut position, velocity): Self::SystemData) {}
}
