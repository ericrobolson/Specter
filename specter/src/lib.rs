extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use inflector::Inflector;

mod object_locator;

pub mod string_generator;
use string_generator::StringGenerator;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

#[derive(Parser)]
#[grammar = "nio.pest"]
struct NioParser;

const file_type: &'static str = ".nio";

/// Build the Specter files
pub fn build() {
    let relevant_files = object_locator::locate_objects(file_type);

    let mut found_language_data = vec![];
    for p in relevant_files {
        let path = p;
        let contents = fs::read_to_string(path).unwrap();

        let nio = parse_nio(contents);
        if nio.is_err() {
            println!("an err!");
        }
        println!("not an err?");

        let data = nio.unwrap();
        found_language_data.push(data);
    }
}

#[derive(Debug)]
pub struct Main {}

impl Parsable for Main {
    fn parse() -> Self {
        //TODO:
        println!("Parse main!");
        Self {}
    }
}

#[derive(Debug)]
pub struct Node {}

impl Parsable for Node {
    fn parse() -> Self {
        //TODO:
        println!("Parse node!");
        Self {}
    }
}

pub trait Parsable
where
    Self: std::marker::Sized,
{
    fn parse() -> Self;
}

#[derive(Debug)]
pub struct LanguageData {
    main: Option<Main>,
    nodes: Vec<Node>,
}
impl LanguageData {
    pub fn new(main: Option<Main>, nodes: Vec<Node>) -> Self {
        return Self {
            main: main,
            nodes: nodes,
        };
    }
}

fn parse_nio(contents: String) -> Result<LanguageData, pest::error::Error<Rule>> {
    let pairs = NioParser::parse(Rule::nio, &contents).unwrap_or_else(|e| panic!("{}", e));

    let mut main = None;
    let mut nodes = vec![];

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::main => {
                    if main.is_some() {
                        panic!("Only one main may be defined!");
                    }

                    main = Some(Main::parse());
                }
                Rule::node => {
                    nodes.push(Node::parse());
                }
                _ => println!("UNIMPLEMENTED!"),
            }
        }
    }
    Ok(LanguageData::new(main, nodes))
}

pub trait Identifiable {
    fn identifier(&self) -> String;
    fn pluralized_identifier(&self) -> String {
        return self.identifier().to_plural();
    }
}

pub trait Rustable {
    fn root_crate(&self) -> String {
        return format!("nio_gen");
    }

    fn get_library_includes() -> String {
        let mut generator = StringGenerator::new();

        return generator.to_string();
    }

    fn get_rust_usage(&self) -> String;
    fn to_rust_definition(&self) -> String;
    fn rust_struct_name(&self) -> String;

    fn compile();
}
