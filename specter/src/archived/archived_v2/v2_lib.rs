extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod components;
use components::Component;

mod systems;
use systems::System;

mod data_types;
use data_types::DataType;

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
    fn pluralized_identifier(&self) -> String {
        return self.identifier().to_plural();
    }
}

pub trait Rustable {
    fn root_crate(&self) -> String {
        return format!("specter_gen");
    }

    fn get_library_includes() -> String {
        let mut generator = StringGenerator::new();

        return generator
            .append("use specs::prelude::*;".to_string())
            .add_line()
            .append("".to_string())
            .to_string();
    }

    fn get_rust_usage(&self) -> String;
    fn to_rust_definition(&self) -> String;
    fn rust_struct_name(&self) -> String;

    fn compile(data: &SpecterData);
}

fn validate_identifiers(objects: &Vec<String>) {
    let mut identities: Vec<String> = vec![];
    for object in objects {
        let exists = (identities.iter().find(|identity| **identity == *object)).is_some();

        if exists {
            panic!("Identifier '{}' already in use!", object);
        }

        identities.push(object.clone());
    }
}

fn validate_generated_data(data: &SpecterData) {
    // Validate identifiers
    let mut identifiers = vec![];
    {
        let mut sys_identifiers: Vec<String> =
            data.systems.iter().map(|sys| sys.identifier()).collect();

        let mut component_identifiers: Vec<String> =
            data.components.iter().map(|c| c.identifier()).collect();

        let mut data_type_identifiers: Vec<String> =
            data.data_types.iter().map(|d| d.identifier()).collect();

        data_type_identifiers.dedup();

        identifiers.append(&mut sys_identifiers);
        identifiers.append(&mut component_identifiers);
        identifiers.append(&mut data_type_identifiers);
    }

    validate_identifiers(&identifiers);
}

#[derive(Debug)]
pub struct SpecterData {
    pub components: Vec<Component>,
    pub systems: Vec<System>,
    pub data_types: Vec<DataType>,
}

impl SpecterData {
    pub fn new(components: Vec<Component>, systems: Vec<System>) -> Self {
        return Self {
            components: components,
            systems: systems,
            data_types: vec![],
        };
    }

    pub fn append(&mut self, other: &mut Self) {
        self.components.append(&mut other.components);
        self.systems.append(&mut other.systems);
        self.data_types.append(&mut other.data_types);
    }

    pub fn validate(&mut self) {
        self.data_types.append(&mut vec![
            DataType::new("number".to_string(), vec![]),
            DataType::new("vec2".to_string(), vec![]),
            DataType::new("vec3".to_string(), vec![]),
        ]);

        validate_generated_data(&self);

        for component in self.components.iter_mut() {
            component.link_data_types(&self.data_types);
        }

        for sys in self.systems.iter_mut() {
            sys.link_components(&self.components);
        }
    }

    pub fn compile_module(path: String, crates: Vec<String>, injectable_code: Option<String>) {
        let mut crates = crates.clone();
        crates.sort();

        let module_path = format!("{}/mod.rs", path);

        let mut f = fs::File::create(module_path).unwrap();
        let mut file = LineWriter::new(f);

        let mut generator = StringGenerator::new();

        generator.append(SpecterData::code_header()).add_line();

        for c in crates {
            generator.append(format!("pub mod {};", c)).add_line();
        }

        if injectable_code.is_some() {
            let injectable = injectable_code.unwrap();

            generator.add_line();
            generator.append(injectable);
        }

        file.write_all(generator.to_string().as_bytes()).unwrap();

        file.flush().unwrap();
    }

    pub fn compile(&mut self) {
        self.validate();

        if Path::new(self.base_path()).exists() {
            fs::remove_dir_all(self.base_path()).unwrap(); // TODO: look into only changing updated files?
        }

        DataType::compile(&self);
        Component::compile(&self);
        System::compile(&self);

        // TODO: link dispatchers/systems

        Self::compile_module(
            self.base_path().to_string(),
            vec![
                "systems".to_string(),
                "components".to_string(),
                "data_types".to_string(),
            ],
            None,
        );
    }

    pub fn base_path(&self) -> &str {
        return "src/specter_gen";
    }

    pub fn code_header() -> String {
        let mut generator = StringGenerator::new();

        generator
            .append("///////////////////////////////////////////////////////////////".to_string())
            .add_line()
            .append("// THIS IS GENERATED CODE AND SHOULD NOT BE MODIFIED BY HAND //".to_string())
            .add_line()
            .append("///////////////////////////////////////////////////////////////".to_string())
            .add_line()
            .to_string()
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
