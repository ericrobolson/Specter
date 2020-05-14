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
