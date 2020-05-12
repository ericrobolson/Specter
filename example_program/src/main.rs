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
		while storage.get("s_kill").is_none() {
			// Print console messages
			let print_vals = storage.get("s_console_out");
			if print_vals.is_some() {
				let print_vals = print_vals.unwrap();
				for val in print_vals {println!("{:?}", val);}
				storage.remove("s_console_out");
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
			if let Some(value_array) = storage.get_mut("s_console_out") {
				let mut vals = &mut *value_array;
				vals.push("Hello world through console".to_string());
			}
			// Otherwise, initialize a new entry in storage
			else {
				storage.insert("s_console_out".to_string(), vec!["Hello world through console".to_string()]);
			}
		}
		
		// Send signal print
		{
			// First, check if there exists a storage entry. If so, add it to the back of existing signals.
			if let Some(value_array) = storage.get_mut("s_print") {
				let mut vals = &mut *value_array;
				vals.push("Hello world through signal".to_string());
			}
			// Otherwise, initialize a new entry in storage
			else {
				storage.insert("s_print".to_string(), vec!["Hello world through signal".to_string()]);
			}
		}
	}
}

pub struct println {
	pub print_signal_index: usize,
}
impl println {
	pub fn new() -> Self {
		Self {
			print_signal_index: 0,
		}
	}
	pub fn execute(&mut self, storage: &mut HashMap<String, Vec<String>>) {
		
		//Check to see that all inputs are ready
		// First, retrieve all relevant inputs
		
		let s_print = storage.get("s_print");
		// TODO: check to make sure that it hasn't been referenced yet using the 'MESSAGE_index' value on the node.
		{
			
			if (s_print.is_none()) || (storage.get("s_print").is_none()) {
				return;
			}
		}
		// If we're here, that means that the node can execute. Get the current values, then increment the current message index for the nodes.
		let s_print = (s_print.unwrap())[self.print_signal_index]; //TODO: wire up message index
		self.print_signal_index += 1;
		
		// Send signal console_out
		{
			// First, check if there exists a storage entry. If so, add it to the back of existing signals.
			if let Some(value_array) = storage.get_mut("s_console_out") {
				let mut vals = &mut *value_array;
				vals.push(print.to_string());
			}
			// Otherwise, initialize a new entry in storage
			else {
				storage.insert("s_console_out".to_string(), vec![print.to_string()]);
			}
		}
		
		// Send signal kill
		{
			// First, check if there exists a storage entry. If so, add it to the back of existing signals.
			if let Some(value_array) = storage.get_mut("s_kill") {
				let mut vals = &mut *value_array;
				vals.push(true.to_string());
			}
			// Otherwise, initialize a new entry in storage
			else {
				storage.insert("s_kill".to_string(), vec![true.to_string()]);
			}
		}
	}
}