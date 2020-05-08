extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use inflector::Inflector;

mod backend;
use backend::{locate_objects, StringGenerator};

mod nio;
use nio::{Identifier, Input, MainNode, Node, Output};

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

#[derive(Parser)]
#[grammar = "nio.pest"]
struct NioParser;

#[derive(Debug, Clone)]
pub struct MetaData {
    file: String,
    start_line_number: usize,
    start_line_position: usize,
}

pub trait MetaDatable {
    fn metadata(&self) -> MetaData;
}

const file_type: &'static str = ".nio";

/// Build the Specter files
pub fn build() {
    let relevant_files = locate_objects(file_type);

    let mut language_data = LanguageData::empty();

    for p in relevant_files {
        let path = p;
        let contents = fs::read_to_string(path).unwrap();

        let data = parse_nio(contents);
        if data.is_err() {
            println!("an err!");
        }

        language_data.join(&data.unwrap());
    }

    language_data.compile(TargetLanguage::Rust);
}

pub trait Parsable
where
    Self: std::marker::Sized,
{
    fn parse(inner_pair: pest::iterators::Pair<'_, Rule>) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub enum TargetLanguage {
    Rust,
}

pub trait Compilable {
    fn link(&self, data: &LanguageData) -> Self;
    fn validate(&self, data: &LanguageData);
    fn compile(&self, target: TargetLanguage, data: &LanguageData) -> String;
}

#[derive(Debug, Clone)]
pub struct LanguageData {
    main: Option<MainNode>,
    nodes: Vec<Node>,
}

impl LanguageData {
    pub fn empty() -> Self {
        return Self::new(None, vec![]);
    }

    pub fn outputs(&self) -> HashMap<String, &nio::identifier::Identifier> {
        let mut hash = HashMap::new();

        if self.main.is_some() {
            self.main
                .as_ref()
                .unwrap()
                .output
                .identifiers
                .iter()
                .for_each(|i| {
                    hash.insert(i.id.clone(), i);
                });
        }

        for identifier in self
            .nodes
            .iter()
            .map(|node| &node.output.identifiers)
            .flatten()
        {
            let prev_value = hash.insert(identifier.id.clone(), &identifier);

            if prev_value.is_some() {
                panic!("Output with id '{}' has been defined twice!", identifier.id);
            }
        }

        return hash;
    }

    fn new(main: Option<MainNode>, nodes: Vec<Node>) -> Self {
        return Self {
            main: main,
            nodes: nodes,
        };
    }

    fn validate(&self) {
        if self.main.is_none() {
            panic!("At least one main is required!");
        }

        self.main.as_ref().unwrap().validate(&self);

        for node in &self.nodes {
            node.validate(&self);
        }
    }

    fn link(&mut self) {
        // Link Main
        {
            if self.main.is_none() {
                panic!("At least one main is required!");
            }
            self.main = Some(self.main.as_ref().unwrap().link(&self));
        }

        // Link nodes
        let linked_nodes = self.nodes.iter().map(|node| node.link(&self)).collect();
        self.nodes = linked_nodes;
    }

    pub fn compile(&mut self, language: TargetLanguage) -> std::string::String {
        self.link();
        self.validate();

        let mut generator = StringGenerator::new();

        // Compile main
        generator
            .append(self.main.as_ref().unwrap().compile(language, &self))
            .add_line();

        // Add nodes
        for node in &self.nodes {
            println!("{:?}", node);
            generator.add_line().append(node.compile(language, &self));
        }

        println!("{}", generator.to_string());

        return String::new();
    }

    pub fn join(&mut self, other: &Self) {
        if self.main.is_some() && other.main.is_some() {
            panic!("Only one main may be defined!");
        }

        if self.main.is_none() {
            self.main = other.main.clone();
        }

        self.nodes.append(&mut other.nodes.clone());
    }
}

fn parse_nio(contents: String) -> Result<LanguageData, pest::error::Error<Rule>> {
    let pairs = NioParser::parse(Rule::program, &contents).unwrap_or_else(|e| panic!("{}", e));

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
