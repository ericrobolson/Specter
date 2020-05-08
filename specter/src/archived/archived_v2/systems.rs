use super::*;

#[derive(Debug, Clone)]
pub struct System {
    identifier: String,
    pub component_references: Vec<SystemComponentReference>,
}

impl System {
    pub fn new(identifier: String, component_references: Vec<SystemComponentReference>) -> Self {
        Self {
            identifier: identifier.to_lowercase(),
            component_references: component_references,
        }
    }

    pub fn link_components(&mut self, components: &Vec<Component>) {
        for component_ref in self.component_references.iter_mut() {
            let matched_component = components
                .iter()
                .find(|c| c.identifier() == component_ref.identifier);

            if matched_component.is_none() {
                panic!("Unable to match component '{}'!", component_ref.identifier);
            }

            component_ref.component = Some((*matched_component.unwrap()).clone());
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemComponentReference {
    pub identifier: String,
    pub writeable: bool,
    pub component: Option<Component>,
    pub referenced_properties: Vec<String>,
}

impl SystemComponentReference {
    pub fn new(identifier: String, writeable: bool, referenced_properties: Vec<String>) -> Self {
        return Self {
            identifier: identifier.to_lowercase(),
            component: None,
            writeable: writeable,
            referenced_properties: referenced_properties,
        };
    }

    fn is_writeable(s: &str) -> bool {
        return s.trim().eq("c-w");
    }
}

impl Rustable for System {
    fn to_rust_definition(&self) -> String {
        let mut generator = StringGenerator::new();

        // Add library references
        generator.append(Self::get_library_includes()).add_lines(2);

        // Add component references
        for component_ref in &self.component_references {
            let component = component_ref.component.as_ref().unwrap();

            generator.append(component.get_rust_usage());
            generator.add_line();
        }

        generator
            .add_line()
            .append(format!("pub struct {};", self.rust_struct_name()))
            .add_lines(2)
            .append(format!(
                "impl<'a> System<'a> for {} ",
                self.rust_struct_name()
            ))
            .append("{".to_string())
            // Implementation
            .indent()
            .add_line()
            .append("type SystemData = (".to_string())
            .indent();

        for component_ref in &self.component_references {
            let component = component_ref.component.as_ref().unwrap();

            generator.add_line();
            if component_ref.writeable {
                generator.append("WriteStorage".to_string());
            } else {
                generator.append("ReadStorage".to_string());
            }

            generator.append("<'a, ".to_string());
            generator.append(component.rust_struct_name());
            generator.append(">,".to_string());
        }

        generator
            .unindent()
            .add_line()
            .append(");".to_string())
            .add_lines(2)
            .append("fn run(&mut self, (".to_string());

        for (i, component_ref) in self.component_references.iter().enumerate() {
            let component = component_ref.component.as_ref().unwrap();

            if component_ref.writeable {
                generator.append("mut ".to_string());
            }

            generator.append(component.pluralized_identifier());

            if i < self.component_references.len() - 1 {
                generator.append(", ".to_string());
            }
        }

        return generator
            .append("): Self::SystemData) {".to_string())
            .add_line()
            .append("}".to_string())
            // End system definition
            .unindent()
            .add_line()
            .append("}".to_string())
            .to_string();
    }

    fn rust_struct_name(&self) -> String {
        return format!("{}System", self.identifier().to_title_case());
    }

    fn get_rust_usage(&self) -> std::string::String {
        return format!(
            "use crate::{}::systems::{}::{};",
            self.root_crate(),
            self.identifier(),
            self.rust_struct_name()
        );
    }

    fn compile(data: &SpecterData) {
        let path = format!("{}/systems", data.base_path());

        fs::create_dir_all(path.clone()).unwrap();

        for system in &data.systems {
            let system_path = format!("{}/{}.rs", path, system.identifier());

            let mut f = fs::File::create(system_path).unwrap();
            let mut file = LineWriter::new(f);

            let mut generator = StringGenerator::new();
            generator
                .append(SpecterData::code_header())
                .add_line()
                .append(system.to_rust_definition());

            file.write_all(generator.to_string().as_bytes()).unwrap();

            file.flush().unwrap();
        }

        let crates = data.systems.iter().map(|obj| obj.identifier()).collect();
        SpecterData::compile_module(path, crates, None);
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
                    let mut component_ref =
                        SystemComponentReference::new(component_def.to_string(), writeable, vec![]);

                    let existing_component = component_refs
                        .iter()
                        .find(|c| c.identifier == component_ref.identifier);

                    if existing_component.is_some() {
                        let existing_component = existing_component.unwrap();
                        // If previously not writeable, update current ref to be writeable
                        if existing_component.writeable && !component_ref.writeable {
                            component_ref.writeable = true;
                        }

                        component_refs.retain(|c| c.identifier != component_ref.identifier);
                    }

                    component_refs.push(component_ref);
                }
                _ => {}
            }
        }
    }

    component_refs.sort_by(|a, b| a.identifier.partial_cmp(&b.identifier).unwrap());

    return System::new(identity.unwrap().to_lowercase(), component_refs);
}
