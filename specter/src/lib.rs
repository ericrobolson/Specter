extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod object_locator;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

#[derive(Parser)]
#[grammar = "specter.pest"]
struct MyParser;

struct StringGenerator {
    indent_count: usize,
    value: String,
}
impl StringGenerator {
    pub fn new() -> Self {
        return Self {
            indent_count: 0,
            value: String::new(),
        };
    }

    pub fn from_string(value: String) -> Self {
        let mut gen = Self::new();

        gen.value = value;

        return gen;
    }

    pub fn indent(&mut self) -> &mut Self {
        self.indent_count += 1;
        return self;
    }

    pub fn unindent(&mut self) -> &mut Self {
        if 0 < self.indent_count {
            self.indent_count -= 1;
        }

        return self;
    }

    pub fn append(&mut self, value: String) -> &mut Self {
        self.value += &value;

        return self;
    }

    pub fn add_lines(&mut self, lines: usize) -> &mut Self {
        for _ in 0..lines {
            self.add_line();
        }

        return self;
    }

    pub fn add_line(&mut self) -> &mut Self {
        self.value += "\n";

        for _ in 0..self.indent_count {
            self.value += "\t";
        }

        return self;
    }

    pub fn to_string(&self) -> String {
        return self.value.clone();
    }
}

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

fn generate_components(data: &SpecterData) {
    let mut f = fs::File::create("src/components.rs").unwrap();
    let mut file = LineWriter::new(f);

    let mut generator = StringGenerator::new();

    for component in &data.components {
        generator.append(component.to_rust());
        generator.add_lines(2);
    }

    file.write_all(generator.to_string().as_bytes()).unwrap();

    file.flush().unwrap();
}

fn generate_systems(data: &SpecterData) {
    let mut f = fs::File::create("src/systems.rs").unwrap();
    let mut file = LineWriter::new(f);

    for system in &data.systems {
        file.write_all(system.to_rust().as_bytes()).unwrap();
    }

    file.flush().unwrap();
}

#[derive(Debug)]
struct SpecterData {
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
        generate_components(&self);
        generate_systems(&self);
    }
}

#[derive(Debug)]
struct System {
    identifier: String,
}

impl System {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier: identifier,
        }
    }
}

impl Rustable for System {
    fn to_rust(&self) -> String {
        let mut generator = StringGenerator::new();

        generator
            .append(format!("pub struct {};", self.identifier()))
            .add_lines(2)
            .append(format!("impl<'a> System<'a> for {} ", self.identifier()))
            .append("{".to_string())
            // Implementation
            .indent()
            .add_line()
            .append("type SystemData = ();".to_string())
            .add_lines(2)
            .append("fn run(&mut self, (): Self::SystemData) {".to_string())
            .add_line()
            .append("}".to_string())
            // End system definition
            .unindent()
            .add_line()
            .append("}".to_string());
        return generator.to_string();
    }
}

impl Identifiable for System {
    fn identifier(&self) -> String {
        return self.identifier.clone();
    }
}

#[derive(Debug)]
struct Component {
    identifier: String,
    pub properties: Vec<Property>,
}

impl Component {
    pub fn new(identifier: String, properties: Vec<Property>) -> Self {
        return Self {
            identifier: identifier,
            properties: properties,
        };
    }
}

impl Rustable for Component {
    fn to_rust(&self) -> String {
        let mut generator = StringGenerator::new();

        generator
            .append(format!("pub struct {} ", self.identifier()))
            .append("{".to_string())
            .indent();

        if self.properties.is_empty() == false {
            for prop in self.properties.iter() {
                generator
                    .add_line()
                    .append(format!("pub {}: {:?},", prop.identifier, prop.prop_type));
            }

            generator.unindent().add_line();
        }

        generator.append("}".to_string());

        generator.to_string()
    }
}

impl Identifiable for Component {
    fn identifier(&self) -> String {
        return self.identifier.clone();
    }
}

#[derive(Debug)]
struct Property {
    pub identifier: String,
    pub default_value: String,
    pub prop_type: Rule,
}

impl Property {
    pub fn new(identifier: String, default_value: String, prop_type: Rule) -> Self {
        return Self {
            identifier: identifier,
            default_value: default_value,
            prop_type: prop_type,
        };
    }
}

fn generate_component(inner_pair: pest::iterators::Pair<'_, Rule>) -> Component {
    let mut component_identity = None;

    let mut properties = vec![];

    let mut prop_identifier = None;

    for inner in inner_pair.into_inner() {
        if component_identity.is_none() {
            component_identity = Some(inner.as_str());
            continue;
        }

        if prop_identifier.is_none() {
            prop_identifier = Some(inner.as_str());
            continue;
        }

        let prop_default_value = inner.as_str();

        let mut prop_type = None;
        for inner in inner.into_inner() {
            prop_type = Some(inner.as_rule());
        }

        let property = Property::new(
            prop_identifier.unwrap().to_string().to_lowercase(),
            prop_default_value.to_string().to_lowercase(),
            prop_type.unwrap(),
        );

        properties.push(property);
        prop_identifier = None; // Reset the identifiers
    }

    let component_identity = component_identity.unwrap().to_lowercase();

    return Component::new(component_identity, properties);
}

fn generate_system(inner_pair: pest::iterators::Pair<'_, Rule>) -> System {
    let mut identity = None;

    for inner in inner_pair.into_inner() {
        if identity.is_none() {
            identity = Some(inner.as_str());
            continue;
        }
        /*
        if prop_identifier.is_none() {
            prop_identifier = Some(inner.as_str());
            continue;
        }

        let prop_default_value = inner.as_str();

        let mut prop_type = None;
        for inner in inner.into_inner() {
            prop_type = Some(inner.as_rule());
        }

        let property = Property::new(
            prop_identifier.unwrap().to_string().to_lowercase(),
            prop_default_value.to_string().to_lowercase(),
            prop_type.unwrap(),
        );

        properties.push(property);
        prop_identifier = None; // Reset the identifiers
        */
    }

    let component_identity = identity.unwrap().to_lowercase();

    return System::new(component_identity);
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
                    generated_components.push(generate_component(inner_pair));
                }
                Rule::system => {
                    generated_systems.push(generate_system(inner_pair));
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
