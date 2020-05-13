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

/// Build the Specter files
pub fn build() {
    //TODO: specify files
    let ast = lexer::execute();
    analyzer::execute(&ast);
    compiler::execute(&ast);
}
