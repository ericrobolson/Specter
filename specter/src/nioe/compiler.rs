use super::*;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

pub fn execute(ast: &Ast) {
    let debug = true;

    let mut generator = StringGenerator::new();

    generator
        .append("fn main() {".to_string())
        .indent()
        .add_line()
        .append("let mut nioe = Nioe::new();".to_string())
        .add_line()
        .append("nioe.execute();".to_string())
        .add_line();

    if debug {
        generator
            .append("//NOTE: this loop is strictly for debugging purposes".to_string())
            .add_line()
            .append("loop {}".to_string());
    }

    generator
        .unindent()
        .add_line()
        .append("}".to_string())
        .add_line()
        .append("pub struct Nioe {".to_string())
        .indent();

    let mut nioe_output_queues = vec![];
    let mut output_structs = vec![];

    match ast {
        Ast::Node(n) => {}
        Ast::Program(nodes) => {
            for node in nodes {
                match node {
                    Ast::Node(n) => {
                        /*
                        //TODO: not sure this belongs here, mainly working on Nioe implementation for now
                        generator
                            .add_line()
                            .append(format!("pub struct {} {{", n.id.rust()))
                            .indent();

                        match &n.input {
                            ast::Inputs::Silent(_) => {
                                // Do nothing for silent inputs
                            }
                            ast::Inputs::References(_, ids) => {
                                for id in ids {
                                    generator.add_line().append(format!(
                                        "pub {}_in_queue: Vec<{}>,",
                                        id.id,
                                        id.rust()
                                    ));
                                }
                            }
                        }
                        */

                        match &n.output {
                            ast::Outputs::Silent(_) => {
                                // Do nothing for silent inputs
                            }
                            ast::Outputs::References(_, ids) => {
                                for id in ids {
                                    nioe_output_queues.push(id.clone());
                                    output_structs.push(id.clone());
                                    /*
                                    //TODO: not sure this belongs here, mainly working on Nioe implementation for now
                                    generator.add_line().append(format!(
                                        "pub {}_out_queue: Vec<{}>,",
                                        id.id,
                                        id.rust()
                                    ));
                                    */
                                }
                            }
                        }
                        /*
                        //TODO: not sure this belongs here, mainly working on Nioe implementation for now
                        generator.unindent().add_line().append("}".to_string());

                        generator
                            .add_line()
                            .append(format!("impl {} {{", n.id.rust()))
                            .add_line()
                            .indent()
                            .add_line()
                            .append("pub fn try_execute(&self) {}".to_string())
                            .unindent()
                            .add_line()
                            .append("}".to_string());
                            */
                    }
                    _ => {}
                }
            }
        }
    }

    for output in &nioe_output_queues {
        generator.add_line().append(format!(
            "pub {}: Vec<{}>",
            output_queue(&output.id),
            output.rust()
        ));
    }

    generator
        .unindent()
        .add_line()
        .append("}".to_string())
        .add_line()
        .append("impl Nioe {".to_string())
        .indent()
        .add_line()
        .append("pub fn new() -> Self {".to_string())
        .indent()
        .add_line()
        .append("return Self{".to_string())
        .indent();
    // Init output queues
    for output in &nioe_output_queues {
        generator
            .add_line()
            .append(format!("{}: vec![],", output_queue(&output.id),));
    }

    generator.unindent().add_line().append("};".to_string());

    generator
        .unindent()
        .add_line()
        .append("}".to_string())
        .add_line()
        .append("pub fn execute(&mut self) {".to_string())
        .indent()
        .add_line()
        .append("//TODO: trigger execution of nodes with a 'silent' input".to_string())
        .unindent()
        .add_line()
        .append("}".to_string())
        .unindent()
        .add_line()
        .append("}".to_string());

    // Generate output structs
    {
        for output in output_structs {
            generator
                .add_line()
                .append(format!("pub struct {} {{}}", output.rust()));
        }
    }

    println!("{}", generator.to_string());

    write_to_disk(generator.to_string());
}

fn output_queue(id: &String) -> String {
    return format!("{}_output_queue", id);
}

fn write_to_disk(code: String) {
    let component_path = format!("src/main.rs");

    let mut f = fs::File::create(component_path).unwrap();
    let mut file = LineWriter::new(f);

    file.write_all(code.as_bytes()).unwrap();

    file.flush().unwrap();
}
