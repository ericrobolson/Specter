fn main() {
	let mut nioe = Nioe::new();
	nioe.execute();
}
pub struct Nioe {
	pub print_output_queue: Vec<n_print>
}
impl Nioe {
	pub fn new() -> Self {
		return Self{
			print_output_queue: vec![],
		};
	}
	pub fn execute(&mut self) {
		//TODO: trigger execution of nodes with a 'silent' input
	}
}