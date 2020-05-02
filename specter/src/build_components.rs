use super::object_locator::{ObjectTypes, SpecterFileObject};

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

use inflector::Inflector;

#[derive(Debug)]
pub enum PropertyType {
    Number,
    Vec2,
    Vec3,
    Str,
}

impl PropertyType {
    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();

        return match s.as_str() {
            "number" => PropertyType::Number,
            "vec2" => PropertyType::Vec2,
            "vec3" => PropertyType::Vec3,
            _ => PropertyType::Str,
        };
    }

    pub fn to_rust(&self) -> &'static str {
        return match self {
            PropertyType::Str => "String",
            PropertyType::Number => "Number",
            PropertyType::Vec2 => "Vec2",
            PropertyType::Vec3 => "Vec3",
        };
    }
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub kind: PropertyType,
}

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub properties: Vec<Property>,
}

impl Component {
    pub fn to_rust(&self) -> String {
        let mut s = format!("pub struct {}Component {{", self.name.to_title_case());

        for prop in &self.properties {
            s = format!("{}\n\tpub {}: {},", s, prop.name, prop.kind.to_rust());
        }

        let s = format!("{}\n}}\n", s);

        return s;
    }
}

pub fn build(objects: &Vec<SpecterFileObject>) -> Vec<Component> {
    let objects = objects.iter().filter(|o| {
        if o.kind == ObjectTypes::Component {
            if o.path.is_none() {
                panic!("Path not attached for component!");
            }

            return true;
        }

        return false;
    });

    let mut components = vec![];

    for object in objects {
        let path_str = object.path.as_ref().unwrap();

        if let Ok(lines) = read_lines(path_str) {
            let mut component_name = None;
            let mut properties = vec![];
            let mut first_line = true;

            for line in lines {
                if let Ok(mut s) = line {
                    s.retain(|c| !c.is_whitespace());
                    if s.is_empty() {
                        continue;
                    }

                    if first_line {
                        // Get the component name
                        component_name = Some(s.replace(":", ""));
                        first_line = false;
                        continue;
                    }

                    // Create properties
                    let prop: Vec<&str> = s.split(':').collect();
                    if prop.len() != 2 {
                        // 2 because there's the prop name, and the prop type
                        panic!("Unable to parse property!");
                    }

                    let prop_name = prop[0];
                    let prop_type = PropertyType::from_str(prop[1]);

                    let property = Property {
                        name: prop_name.to_string(),
                        kind: prop_type,
                    };
                    properties.push(property);
                }
            }

            if component_name.is_some() {
                let component = Component {
                    name: component_name.unwrap().to_lowercase(),
                    properties: properties,
                };

                components.push(component);
            }
        }
    }

    return components;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
