// THIS IS AN AUTOGENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND
use std::collections::HashMap;

fn main() {
	let mut nioe = Nioe::new();
	nioe.execute();
	
}

pub struct Nioe {}
impl Nioe {
	pub fn new() -> Self {
		return Self{
		};
	}
	
	pub fn execute(&mut self) {
		// Init storage
		let mut storage: HashMap<String, Vec<String>> = HashMap::new();
		
		// Init nodes
		let mut node_main = main::new();
		let mut node_println = println::new();
		
		// Begin execution of inputless nodes
		node_main.execute(&mut storage);
		
		// This is the core loop that processes node i/o
		while storage.is_empty() == false {
			node_println.execute(&mut storage);
		}
	}
}

pub struct main {
}
impl main {
	pub fn new() -> Self {
		Self {
		}
	}
	pub fn execute(&mut self, storage: &mut HashMap<String, Vec<String>>) {
		println!("TODO: main.execute()");
	}
}

pub struct println {
	pub print_index: usize,
}
impl println {
	pub fn new() -> Self {
		Self {
			print_index: 0,
		}
	}
	pub fn execute(&mut self, storage: &mut HashMap<String, Vec<String>>) {
		println!("TODO: println.execute()");
	}
}