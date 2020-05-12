// THIS IS AN AUTOGENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
	let mut nioe = Nioe::new();
	nioe.execute();
	// NOTE: This loop is in place for debugging
	println!("Press enter to exit.");
	io::stdin().lock().lines().next().unwrap().unwrap();
	
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
		while storage.get("kill").is_none() {
			// Print console messages
			let print_vals = storage.get("console_out");
			if print_vals.is_some() {
				let print_vals = print_vals.unwrap();
				for val in print_vals {println!("{:?}", val);}
				storage.remove("console_out");
			}
			
			// Node executions
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
		
		
		// Send signal console_out
		{
			// First, check if there exists a storage entry. If so, add it to the back of existing signals.
			if let Some(value_array) = storage.get_mut("console_out") {
				let mut vals = &mut *value_array;
				vals.push("Hello world through console".to_string());
			}
			// Otherwise, initialize a new entry in storage
			else {
				storage.insert("console_out".to_string(), vec!["Hello world through console".to_string()]);
			}
		}
		
		// Send signal print
		{
			// First, check if there exists a storage entry. If so, add it to the back of existing signals.
			if let Some(value_array) = storage.get_mut("print") {
				let mut vals = &mut *value_array;
				vals.push("Hello world through signal".to_string());
			}
			// Otherwise, initialize a new entry in storage
			else {
				storage.insert("print".to_string(), vec!["Hello world through signal".to_string()]);
			}
		}
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
		
		//Check to see that all inputs are ready
		// TODO: check to make sure that it hasn't been referenced yet using the 'MESSAGE_index' value on the node.
		if storage.get("print").is_none() {
			return;
		}
		
		// Send signal kill
		{
			// First, check if there exists a storage entry. If so, add it to the back of existing signals.
			if let Some(value_array) = storage.get_mut("kill") {
				let mut vals = &mut *value_array;
				vals.push(true.to_string());
			}
			// Otherwise, initialize a new entry in storage
			else {
				storage.insert("kill".to_string(), vec![true.to_string()]);
			}
		}
	}
}