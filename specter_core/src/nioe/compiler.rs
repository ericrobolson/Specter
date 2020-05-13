use super::*;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

const END_PROGRAM: &'static str = "kill";
const PRINT_SIGNAL: &'static str = "s_console_out";

fn add_main(code: String) -> String {
    let mut generator = StringGenerator::from_string(code);
    let mut main_generator = StringGenerator::new();

    main_generator
        .append("// THIS IS AN AUTOGENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND".to_string())
        .add_line()
        .append("use std::collections::HashMap;".to_string())
        .add_line()
        .append("use std::io::{self, BufRead};".to_string())
        .add_lines(2)
        .append("fn main() {".to_string())
        .indent()
        .add_line()
        .append("let mut nioe = Nioe::new();".to_string())
        .add_line()
        .append("nioe.execute();".to_string())
        .add_line()
        .append("// NOTE: This loop is in place for debugging".to_string())
        .add_line()
        .append("println!(\"Press enter to exit.\");".to_string())
        .add_line()
        .append("io::stdin().lock().lines().next().unwrap().unwrap();".to_string())
        .add_line();
    main_generator
        .unindent()
        .add_line()
        .append("}".to_string())
        .add_line();

    generator.prepend(main_generator.to_string());

    return generator.to_string();
}

fn generate(ast: &Ast) -> String {
    let mut nioe_output_queues = vec![];
    let mut output_structs = vec![];
    let mut inputless_nodes = vec![];
    let mut triggered_nodes = vec![];
    let mut nodes_refs = vec![];

    match ast {
        Ast::Node(n) => {}
        Ast::Program(nodes) => {
            for node in nodes {
                match node {
                    Ast::Node(n) => {
                        if n.input.is_none() {
                            //wire up inputless node
                            inputless_nodes.push(n.clone());
                        } else {
                            triggered_nodes.push(n.clone());
                        }

                        nodes_refs.push(n.clone());

                        match &n.output {
                            ast::Outputs::References(_, ids) => {
                                for id in ids {
                                    nioe_output_queues.push(id.clone());
                                    output_structs.push(id.clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut generator = StringGenerator::new();
    // Generate nioe struct/impl
    {
        generator
            .add_line()
            .append("#[derive(PartialEq)]".to_string())
            .add_line()
            .append("pub enum KillStates {".to_string())
            .append("NotSignalled, Kill".to_string())
            .append("}".to_string())
            .add_line()
            .append("pub struct Nioe {}".to_string())
            .add_line()
            .append("impl Nioe {".to_string())
            .indent()
            .add_line()
            .append("pub fn new() -> Self {".to_string())
            .indent()
            .add_line()
            .append("return Self{".to_string())
            .indent()
            .unindent()
            .add_line()
            .append("};".to_string());

        // Begin execution method
        {
            generator
                .unindent()
                .add_line()
                .append("}".to_string())
                .add_lines(2)
                .append("pub fn execute(&mut self) {".to_string())
                .indent()
                .add_line()
                // Init message storage
                .append("// Init storage".to_string())
                .add_line()
                .append(
                    "let mut storage: HashMap<String, Vec<String>> = HashMap::new();".to_string(),
                )
                .add_lines(2)
                .append("// Init nodes".to_string());

            // Create nodes
            for n in &nodes_refs {
                generator.add_line().append(format!(
                    "let mut {} = {}::new();",
                    alias(&n),
                    n.id.rust()
                ));
            }

            // Trigger execution of inputless nodes
            generator
                .add_lines(2)
                .append("// Begin execution of inputless nodes".to_string());
            for n in inputless_nodes {
                generator
                    .add_line()
                    .append(format!("{}.execute(&mut storage);", alias(&n)));
            }

            // Add a simple 'loop' for processing of messages while they need to be processed
            {
                generator
                    .add_lines(2)
                    .append("// This is the core loop that processes node i/o".to_string())
                    .add_line()
                    .append("let mut kill_state = KillStates::NotSignalled;".to_string())
                    .add_line()
                    .append("while kill_state != KillStates::Kill {".to_string())
                    .indent();

                // For each node, link it up
                generator
                    .add_lines(2)
                    .append("// Node executions".to_string());
                for n in triggered_nodes {
                    generator
                        .add_line()
                        .append(format!("{}.execute(&mut storage);", alias(&n)));
                }

                // Handle end conditions
                {
                    generator
                        .add_lines(2)
                        .append("// Check whether the program should be killed. If it's just been signalled, do one more execution pass for any outstanding stuff.".to_string())
                        .add_line()
                        .append(format!("if {}.is_some() {{", storage_get(&END_PROGRAM.to_string())))
                        .indent()
                        .add_line()
                        .append("kill_state = KillStates::Kill;".to_string())
                        .unindent()
                        .add_line()
                        .append("}".to_string());
                }

                // Handle console_out
                {
                    generator
                        .add_line()
                        .append("// Print console messages".to_string())
                        .add_line()
                        .append(format!(
                            "let print_vals = storage.get(\"{}\");",
                            PRINT_SIGNAL
                        ))
                        .add_line()
                        .append("if print_vals.is_some() {".to_string())
                        .indent()
                        .add_line()
                        // do print
                        .append("let print_vals = print_vals.unwrap();".to_string())
                        .add_line()
                        .append("for val in print_vals {println!(\"{:?}\", val);}".to_string())
                        .add_line()
                        .append(format!("storage.remove(\"{}\");", PRINT_SIGNAL))
                        .unindent()
                        .add_line()
                        .append("}".to_string());
                }
                // Close loop
                generator.unindent().add_line().append("}".to_string());
            }

            // Close the brackets
            generator
                .unindent()
                .add_line()
                .append("}".to_string())
                .unindent()
                .add_line()
                .append("}".to_string());
        }
    }

    // Generate node structs
    {
        for node in &nodes_refs {
            generator
                .add_lines(2)
                .append(format!("pub struct {} {{", node.id.rust()))
                .indent();

            // For each input, add a counter so it knows what input position it's on in the storage key
            if node.input.is_some() {
                let input = node.input.as_ref().unwrap();
                match input {
                    ast::Inputs::References(_, reference_ids) => {
                        for signal_ref in reference_ids {
                            generator
                                .add_line()
                                .append(format!("pub {}: usize,", reference_counter(signal_ref)));
                        }
                    }
                }
            }

            generator
                .unindent()
                .add_line()
                .append("}".to_string())
                .add_line()
                // Create the implementation
                .append(format!("impl {} {{", node.id.rust()))
                .indent()
                .add_line()
                .append("pub fn new() -> Self {".to_string())
                .indent()
                .add_line()
                .append("Self {".to_string())
                .indent();
            // For each reference index, initialize it
            if node.input.is_some() {
                let input = node.input.as_ref().unwrap();
                match input {
                    ast::Inputs::References(_, reference_ids) => {
                        for signal_ref in reference_ids {
                            generator
                                .add_line()
                                .append(format!("{}: 0,", reference_counter(signal_ref)));
                        }
                    }
                }
            }
            generator
                .unindent()
                .add_line()
                .append("}".to_string())
                .unindent()
                .add_line()
                .append("}".to_string());
            // Execution function
            {
                generator
                    .add_line()
                    // Execution function
                    .append(
                        "pub fn execute(&mut self, storage: &mut HashMap<String, Vec<String>>) {"
                            .to_string(),
                    )
                    .indent()
                    .add_line();

                // If none of the inputs are ready, don't continue execution
                {
                    if node.input.is_some() {
                        let input = node.input.as_ref().unwrap().clone();
                        match input {
                            ast::Inputs::References(_, refs) => {
                                generator
                                    .add_line()
                                    .append("//Check to see that all inputs are ready".to_string())
                                    .add_line()
                                    .append("// First, retrieve all relevant inputs".to_string())
                                    .add_line();

                                let mut signal_aliases = vec![];

                                for reference in &refs {
                                    let signal_alias = format!("s_{}", reference.rust());
                                    signal_aliases.push((signal_alias.clone(), reference.clone()));

                                    generator.add_line().append(format!(
                                        "let {} = {};",
                                        signal_alias,
                                        storage_get(&reference.rust())
                                    ));
                                }

                                generator.add_line().append("// TODO: check to make sure that it hasn't been referenced yet using the 'MESSAGE_index' value on the node.".to_string())
                                    .add_line()
                                    .append("{".to_string())
                                    .indent()
                                    .add_line();

                                // For each input, check to see that it's in storage
                                {
                                    generator.add_line();
                                    let mut all_entries_exist_code = StringGenerator::new();
                                    {
                                        let mut reference_count = 0;
                                        for reference in &signal_aliases {
                                            if reference_count != 0 {
                                                all_entries_exist_code.append(" || ".to_string());
                                            }

                                            reference_count += 1;

                                            all_entries_exist_code
                                                .append(format!("{}.is_none()", reference.0));
                                        }
                                    }

                                    let mut new_inputs_not_present_code = StringGenerator::new();
                                    {
                                        let mut reference_count = 0;
                                        for reference in &refs {
                                            if reference_count != 0 {
                                                new_inputs_not_present_code
                                                    .append(" || ".to_string());
                                            }

                                            reference_count += 1;

                                            new_inputs_not_present_code.append(format!(
                                                "{}.is_none()",
                                                storage_get(&reference.rust())
                                            ));
                                        }
                                    }

                                    generator
                                        .append(format!(
                                            "if ({}) || ({})",
                                            all_entries_exist_code.to_string(),
                                            new_inputs_not_present_code.to_string()
                                        ))
                                        .append(" {".to_string())
                                        .indent()
                                        .add_line()
                                        .append("return;".to_string())
                                        .unindent()
                                        .add_line()
                                        .append("}".to_string())
                                        .unindent()
                                        .add_line()
                                        .append("}".to_string());

                                    // If we're here, then that means it can process.
                                    generator
                                        .add_line()
                                        .append("// If we're here, that means that the node can execute. Get the current values, then increment the current message index for the nodes.".to_string());

                                    for alias in &signal_aliases {
                                        generator.add_line().append(format!(
                                            "let {} = ({}.unwrap())[self.{}].clone();",
                                            alias.0,
                                            alias.0,
                                            reference_counter(&alias.1)
                                        ));

                                        generator.add_line().append(format!(
                                            "self.{} += 1;",
                                            reference_counter(&alias.1)
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }

                // Generate the actual execution implementation
                {
                    for statement in &node.execute.statements {
                        match statement {
                            ast::ExecuteStatements::Signal(s) => {
                                signal(s, &mut generator);
                            }
                            _ => unimplemented!("Compiler: statement compiling"),
                        }
                    }
                }

                generator.unindent().add_line().append("}".to_string());
            }

            generator.unindent().add_line().append("}".to_string());
        }
    }

    return generator.to_string();
}

fn storage_get(rust_id: &String) -> String {
    return format!("storage.get(\"s_{}\")", rust_id);
}

fn storage_get_mut(rust_id: &String) -> String {
    return format!("storage.get_mut(\"s_{}\")", rust_id);
}

fn signal(s: &ast::Signal, generator: &mut StringGenerator) {
    let value = match &s.message_value.value {
        ast::PrimitiveTypes::Number(n) => format!("{}", n),
        ast::PrimitiveTypes::Boolean(b) => format!("{}", b),
        ast::PrimitiveTypes::Reference(i) => format!("s_{}", i.rust()),
        ast::PrimitiveTypes::Str(s) => s.clone(),
    };
    let rust_id = s.id.rust();

    generator
        .add_line()
        .append(format!("// Send signal {}", s.id.rust()))
        .add_line()
        .append("{".to_string())
        .indent()
        .add_line()
        .append("// First, check if there exists a storage entry. If so, add it to the back of existing signals.".to_string())
        .add_line()
        .append(format!(
            "if let Some(value_array) = {} {{",
            storage_get_mut(&rust_id)
        ))
        .indent()
        .add_line()
        .append("let mut vals = &mut *value_array;".to_string())
        .add_line()
        .append(format!("vals.push({}.to_string());", value))
        .unindent()
        .add_line()
        .append("}".to_string())
        .add_line()
        .append("// Otherwise, initialize a new entry in storage".to_string())
        .add_line()
        .append("else {".to_string())
        .indent()
        .add_line()
        .append(format!(
            "storage.insert(\"s_{}\".to_string(), vec![{}.to_string()]);",
            rust_id, value
        ))
        .unindent()
        .add_line()
        .append("}".to_string())
        .unindent()
        .add_line()
        .append("}".to_string());

    return;
}

fn reference_counter(id: &nioe::ast::Identifier) -> String {
    return format!("{}_signal_index", id.rust());
}
fn alias(node: &ast::Node) -> String {
    return format!("node_{}", node.id.rust());
}

pub fn execute(ast: &Ast, source_path: &String) {
    let transpiled_code = generate(ast);
    let finalized_code = add_main(transpiled_code);
    write_to_disk(finalized_code, &source_path);
}

fn write_to_disk(code: String, source_path: &String) {
    let debug = false;

    let gen_src_folder = "gen/src";
    fs::create_dir_all(gen_src_folder).unwrap();

    if debug {
        println!("Writing code...");
    }

    let component_path = format!("{}/main.rs", gen_src_folder);
    write_bytes_to_disk(&component_path, code.as_bytes());

    // Write cargo stuff
    let cargo = include_bytes!("gen_code/cargo.gen");
    write_bytes_to_disk(&"gen/Cargo.toml".to_string(), cargo);

    // Build generated code
    env::set_current_dir(format!("{}/gen", source_path)).unwrap();
    let output = std::process::Command::new("cargo")
        .args(&["build"])
        .output()
        .expect("failed to execute");
    if debug {
        println!("Build output: {:?}", output);
    }

    // Copy generated executable
    env::set_current_dir(format!("{}", source_path)).unwrap();
    let exe_name = "nioe.exe";
    let target_exe_name = "nioe.exe";
    fs::copy(format!("gen\\target\\debug\\{}", exe_name), target_exe_name).unwrap();
}

fn write_bytes_to_disk(path: &String, bytes: &[u8]) {
    let mut f = fs::File::create(path).unwrap();
    let mut file = LineWriter::new(f);

    file.write_all(bytes).unwrap();

    file.flush().unwrap();
}
