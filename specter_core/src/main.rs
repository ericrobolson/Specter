// Std defines
use std::collections::HashMap;
use std::fs;

// Pest parser
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "nioe.pest"]
struct NioeParser;

// Custom modules
mod backend;
use backend::{locate_objects, StringGenerator};

mod nioe;
use nioe::{analyzer, compiler, lexer};

const FILE_TYPE: &'static str = ".nioe";

fn main() {
    /*
    Compiler asks
        0) No cmd args shows the 'help' menu
        1) Specify folder for files to parse
        2) Specify optional debug flag which shows transpiled code
    */
    println!("TODO!");
}

/// Build the Specter files
pub fn build() {
    //TODO: specify files
    let ast = lexer::execute();
    analyzer::execute(&ast);
    compiler::execute(&ast);
}
