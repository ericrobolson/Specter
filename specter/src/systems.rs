use super::*;

#[derive(Debug)]
pub struct System {
    identifier: String,
    pub component_references: Vec<SystemComponentReference>,
}

impl System {
    pub fn new(identifier: String, component_references: Vec<SystemComponentReference>) -> Self {
        Self {
            identifier: identifier,
            component_references: component_references,
        }
    }
}

#[derive(Debug)]
pub struct SystemComponentReference {
    pub identifier: String,
    pub writeable: bool,
    pub referenced_properties: Vec<String>,
}

impl SystemComponentReference {
    pub fn new(identifier: String, writeable: bool, referenced_properties: Vec<String>) -> Self {
        return Self {
            identifier: identifier,
            writeable: writeable,
            referenced_properties: referenced_properties,
        };
    }

    pub fn is_writeable(s: &str) -> bool {
        return s.trim().eq("c-w");
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
            .append("type SystemData = (".to_string())
            .indent();

        for component_ref in &self.component_references {
            generator.add_line();
            if component_ref.writeable {
                generator.append("WriteStorage".to_string());
            } else {
                generator.append("ReadStorage".to_string());
            }

            generator.append("<'a, ".to_string());

            generator.append(component_ref.identifier.clone());

            generator.append(">,".to_string());
        }

        generator
            .unindent()
            .add_line()
            .append(");".to_string())
            .add_lines(2)
            .append("fn run(&mut self, (".to_string());

        for (i, component_ref) in self.component_references.iter().enumerate() {
            if component_ref.writeable {
                generator.append("mut ".to_string());
            }
            generator.append(component_ref.identifier.clone());

            if i < self.component_references.len() - 1 {
                generator.append(", ".to_string());
            }
        }

        return generator
            .append(") : Self::SystemData) {".to_string())
            .add_line()
            .append("}".to_string())
            // End system definition
            .unindent()
            .add_line()
            .append("}".to_string())
            .to_string();
    }
}

impl Identifiable for System {
    fn identifier(&self) -> String {
        return self.identifier.clone();
    }
}

pub fn parse_system(inner_pair: pest::iterators::Pair<'_, Rule>) -> System {
    let mut identity = None;

    let mut component_refs: Vec<SystemComponentReference> = vec![];

    for inner in inner_pair.into_inner() {
        if identity.is_none() {
            identity = Some(inner.as_str());
            continue;
        }

        for inner in inner.into_inner() {
            match inner.as_rule() {
                Rule::system_component_alias => {
                    let s = inner.as_str().to_string();

                    let splits: Vec<&str> = s.split(':').collect();

                    let alias = splits[0];
                    println!("alias: {}", alias); // TODO: what to do with the alias?

                    let type_def = splits[1].to_string();

                    let splits: Vec<&str> = type_def.split('.').collect();
                    let read_type = splits[0];
                    let component_def = splits[1];
                    let writeable = SystemComponentReference::is_writeable(read_type);
                    let component_ref =
                        SystemComponentReference::new(component_def.to_string(), writeable, vec![]);

                    component_refs.push(component_ref);
                }
                _ => {}
            }

            //println!("{:?}", inner.as_rule());
            //println!("{}", inner.as_str());
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

    component_refs.sort_by(|a, b| a.identifier.partial_cmp(&b.identifier).unwrap());

    return System::new(identity.unwrap().to_lowercase(), component_refs);
}

pub fn compile(data: &SpecterData) {
    let mut generator = StringGenerator::new();
    for system in &data.systems {
        generator.append(system.to_rust()).add_lines(2);
    }

    let mut f = fs::File::create("src/systems.rs").unwrap();
    let mut file = LineWriter::new(f);

    file.write_all(generator.to_string().as_bytes()).unwrap();

    file.flush().unwrap();
}
