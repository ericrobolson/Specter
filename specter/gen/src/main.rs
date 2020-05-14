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

#[derive(PartialEq)]
pub enum KillStates {NotSignalled, Kill}
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
		// Begin STD lib initializations
		let mut node_std_cout = std_cout::new();
		
		// Begin execution of inputless nodes
		
		// Begin execution of inputless STD nodes
		
		// This is the core loop that processes node i/o
		let mut kill_state = KillStates::NotSignalled;
		while kill_state != KillStates::Kill {
			
			// Node executions
			
			// STD Node executions
			node_std_cout.execute(&mut storage);
			
			// Check whether the program should be killed. If it's just been signalled, do one more execution pass for any outstanding stuff.
			if storage.get("s_kill").is_some() {
				kill_state = KillStates::Kill;
			}
			// Print console messages
			let print_vals = storage.get("s_cout");
			if print_vals.is_some() {
				let print_vals = print_vals.unwrap();
				for val in print_vals {println!("{:?}", val);}
				storage.remove("s_cout");
			}
		}
	}
}

// Console in
// File open
// File save

/// STD Cout node
pub struct std_cout {
    pub cout_signal_index: usize,
}
impl std_cout {
    pub fn new() -> Self {
        Self {
            cout_signal_index: 0,
        }
    }
    pub fn execute(&mut self, storage: &mut std::collections::HashMap<String, Vec<String>>) {
        loop {
            //Check to see that all inputs are ready
            const SIGNAL: &'static str = "std::cout";

            let s_print = storage.get(SIGNAL);
            {
                if s_print.is_none() {
                    return;
                }

                // Now check that the SIGNAL_INDEX is less than the current size, if so then keep processing and execute on that message
                let s_print_collection = s_print.unwrap();
                if s_print_collection.len() <= self.cout_signal_index {
                    return;
                }
            }

            // If we're here, that means that the node can execute. Get the current values, then increment the current message index for the nodes.
            let s_print = (s_print.unwrap())[self.cout_signal_index].clone();
            self.cout_signal_index += 1;

            // Do the actual system execution
            println!("{}", s_print);
        }
    }
}
