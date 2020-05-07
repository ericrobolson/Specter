extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use inflector::Inflector;

mod object_locator;

mod rule_parser;

mod node;
use node::Node;

mod main_node;
use main_node::MainNode;

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

    let mut language_data = LanguageData::new(None, vec![]);
    for p in relevant_files {
        let path = p;
        let contents = fs::read_to_string(path).unwrap();

        let nio = parse_nio(contents);
        if nio.is_err() {
            println!("an err!");
        }

        let data = nio.unwrap();

        language_data = language_data.join(&data);
    }
}

pub trait Parsable
where
    Self: std::marker::Sized,
{
    fn parse(inner_pair: pest::iterators::Pair<'_, Rule>) -> Self;
}

pub enum TargetLanguage {
    Rust,
}

pub trait Compilable {
    fn validate(&self);
    fn compile(&self, target: TargetLanguage);
}

#[derive(Debug, Clone)]
pub struct LanguageData {
    main: Option<MainNode>,
    nodes: Vec<Node>,
}

impl LanguageData {
    pub fn new(main: Option<MainNode>, nodes: Vec<Node>) -> Self {
        return Self {
            main: main,
            nodes: nodes,
        };
    }

    fn validate(&self) {
        unimplemented!();
    }

    fn compile(&self) {
        self.validate();
        unimplemented!();
    }

    pub fn join(&self, other: &Self) -> Self {
        if self.main.is_some() && other.main.is_some() {
            panic!("Only one main may be defined!");
        }

        let mut main = None;
        if self.main.is_some() {
            main = self.main.clone();
        } else if other.main.is_some() {
            main = other.main.clone();
        }

        let mut nodes = self.nodes.clone();
        nodes.append(&mut other.nodes.clone());

        Self {
            main: main,
            nodes: nodes,
        }
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

                    main = Some(MainNode::parse(inner_pair));
                }
                Rule::node => {
                    nodes.push(Node::parse(inner_pair));
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
