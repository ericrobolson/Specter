fn main() {
	let mut nioe = Nioe::new();
	nioe.execute();
	//NOTE: this loop is strictly for debugging purposes
	loop {}
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
pub struct n_print {}