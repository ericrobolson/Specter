extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use inflector::Inflector;

mod backend;
use backend::{locate_objects, StringGenerator};

mod nioe;
use nioe::Ast;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

#[derive(Parser)]
#[grammar = "nioe.pest"]
struct NioeParser;

const file_type: &'static str = ".nioe";

/// Build the Specter files
pub fn build() {
    let relevant_files = locate_objects(file_type);

    for p in relevant_files {
        let path = p;
        let contents = fs::read_to_string(path.clone()).unwrap();

        let data = parse_nioe(path, contents);
        if data.is_err() {
            println!("an err!");
        }

        println!("{:?}", data);
    }
}

fn parse_nioe(path: String, contents: String) -> Result<Ast, pest::error::Error<Rule>> {
    let pairs = NioeParser::parse(Rule::program, &contents).unwrap_or_else(|e| panic!("{}", e));

    let ast = Ast::build(&path, pairs)?;

    Ok(ast)
}
