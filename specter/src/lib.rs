extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod components;
use components::Component;

mod systems;
use systems::System;

mod object_locator;

pub mod string_generator;
use string_generator::StringGenerator;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

#[derive(Parser)]
#[grammar = "specter.pest"]
struct MyParser;

/// Build the Specter files
pub fn build() {
    let objects = object_locator::locate_objects();

    let mut generated_data = SpecterData::new(vec![], vec![]);

    for object in objects {
        if object.path.is_none() {
            continue;
        }

        let path = object.path.unwrap();
        let contents = fs::read_to_string(path).unwrap();

        #[cfg(feature = "debug")]
        {
            println!("Contents: {}", contents);
        }

        let res = parse_specter(contents);

        let mut data = res.unwrap();
        generated_data.append(&mut data);
    }

    generated_data.compile();
}

pub trait Identifiable {
    fn identifier(&self) -> String;
}

pub trait Rustable {
    fn to_rust(&self) -> String;
}

fn validate_identifiers(objects: &Vec<Box<&dyn Identifiable>>) {
    let mut identities: Vec<String> = vec![];
    for object in objects {
        let id = object.identifier();

        let exists = (identities.iter().find(|identity| **identity == id)).is_some();

        if exists {
            panic!("Identifier '{}' already in use!", id);
        }

        identities.push(id);
    }
}

fn validate_generated_data(data: &SpecterData) {
    // Validate identifiers
    let mut identifiers = vec![];
    {
        let mut sys_identifiers: Vec<Box<&dyn Identifiable>> = data
            .systems
            .iter()
            .map(|sys| Box::new(sys as &dyn Identifiable))
            .collect();

        let mut component_identifiers: Vec<Box<&dyn Identifiable>> = data
            .components
            .iter()
            .map(|c| Box::new(c as &dyn Identifiable))
            .collect();

        identifiers.append(&mut sys_identifiers);
        identifiers.append(&mut component_identifiers);
    }

    validate_identifiers(&identifiers);
}

#[derive(Debug)]
pub struct SpecterData {
    pub components: Vec<Component>,
    pub systems: Vec<System>,
}

impl SpecterData {
    pub fn new(components: Vec<Component>, systems: Vec<System>) -> Self {
        return Self {
            components: components,
            systems: systems,
        };
    }

    pub fn append(&mut self, other: &mut Self) {
        self.components.append(&mut other.components);
        self.systems.append(&mut other.systems);
    }

    pub fn compile(&self) {
        validate_generated_data(&self);
        components::compile(&self);
        systems::compile(&self);
    }
}

fn parse_specter(contents: String) -> Result<SpecterData, pest::error::Error<Rule>> {
    #[cfg(feature = "debug")]
    {
        for c in contents.chars() {
            println!("char {}: {:?}", c, c as u32);
        }
    }

    let pairs = MyParser::parse(Rule::specter, &contents)?;

    let mut generated_components = vec![];
    let mut generated_systems = vec![];

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    generated_components.push(components::parse_component(inner_pair));
                }
                Rule::system => {
                    generated_systems.push(systems::parse_system(inner_pair));
                }
                _ => println!("UNIMPLEMENTED!"),
            }
        }

        /*
        #[cfg(feature = "debug")]
        {
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.as_span());
            println!("Text:    {:?}", pair.as_str());

            match pair.as_rule() {
                Rule::specter => println!("SPC {}", pair.as_str()),
                _ => println!("{}", pair.as_str()),
            }

            // A pair can be converted to an iterator of the tokens which make it up:
            for inner_pair in pair.into_inner() {
                println!("inner: {}", inner_pair.as_str());
                match inner_pair.as_rule() {
                    Rule::specter => println!("SPECTR: {}", inner_pair.as_str()),
                    Rule::component => println!("Component: {}", inner_pair.as_str()),
                    Rule::system => println!("System: {}", inner_pair.as_str()),
                    Rule::identifier => println!("Identifier:  {}", inner_pair.as_str()),
                    Rule::stype => println!("STYPE:   {}", inner_pair.as_str()),
                    _ => println!("dafu? {:?}", inner_pair.as_str()),
                };
            }
        }*/
    }
    Ok(SpecterData::new(generated_components, generated_systems))
}
