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

/// Build the Specter files
pub fn build() {
    let objects = object_locator::locate_objects();

    let mut generated_data = SpecterData::new(vec![]);

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

fn validate_generated_data(data: &SpecterData) {
    // Validate the generated components
    {
        let mut component_identities: Vec<String> = vec![];
        for component in &data.components {
            let exists = (component_identities
                .iter()
                .find(|identity| **identity == component.identifier))
            .is_some();

            if exists {
                panic!(
                    "Component '{}' has already been defined!",
                    component.identifier
                );
            }

            component_identities.push(component.identifier.clone());
        }
    }
}

fn generate_components(data: &SpecterData) {
    let mut f = fs::File::create("src/components.rs").unwrap();
    let mut file = LineWriter::new(f);

    for component in &data.components {
        file.write_all(component.to_rust().as_bytes()).unwrap();
    }

    file.flush().unwrap();
}

#[derive(Debug)]
struct SpecterData {
    pub components: Vec<Component>,
}

impl SpecterData {
    pub fn new(components: Vec<Component>) -> Self {
        return Self {
            components: components,
        };
    }

    pub fn append(&mut self, other: &mut Self) {
        self.components.append(&mut other.components);
    }

    pub fn compile(&self) {
        validate_generated_data(&self);
        generate_components(&self);
    }
}

#[derive(Debug)]
struct Component {
    pub identifier: String,
    pub properties: Vec<Property>,
}

impl Component {
    pub fn new(identifier: String, properties: Vec<Property>) -> Self {
        return Self {
            identifier: identifier,
            properties: properties,
        };
    }

    pub fn to_rust(&self) -> String {
        let mut props = String::new();

        for prop in self.properties.iter() {
            props = format!(
                "{}{}",
                props,
                (format!("pub {}: {:?},", prop.identifier, prop.prop_type))
            );
        }

        let impls = format!(
            "impl Component for {} {{ type Storage = VecStorage<Self>;}}",
            self.identifier
        );

        format!(
            "pub struct {} {{ {} }}\n{}\n\n",
            self.identifier, props, impls
        )
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

fn parse_specter(contents: String) -> Result<SpecterData, pest::error::Error<Rule>> {
    #[cfg(feature = "debug")]
    {
        for c in contents.chars() {
            println!("char {}: {:?}", c, c as u32);
        }
    }

    let pairs = MyParser::parse(Rule::specter, &contents)?;

    let mut generated_components = vec![];

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => {
                    generated_components.push(generate_component(inner_pair));
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
    Ok(SpecterData::new(generated_components))
}
