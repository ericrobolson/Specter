use super::*;

#[derive(Debug, Clone)]
pub struct Component {
    identifier: String,
    pub properties: Vec<Property>,
}

impl Component {
    pub fn new(identifier: String, properties: Vec<Property>) -> Self {
        return Self {
            identifier: identifier.to_lowercase(),
            properties: properties,
        };
    }

    pub fn link_data_types(&mut self, data_types: &Vec<DataType>) {
        for prop_ref in self.properties.iter_mut() {
            let matched_prop = data_types
                .iter()
                .find(|c| c.identifier() == prop_ref.prop_type);

            if matched_prop.is_none() {
                panic!("Unable to match data type '{}'!", prop_ref.prop_type);
            }

            prop_ref.data_type = Some((*matched_prop.unwrap()).clone());
        }
    }
}

impl Rustable for Component {
    fn to_rust_definition(&self) -> String {
        let mut generator = StringGenerator::new();

        // Add library references
        generator.append(Self::get_library_includes()).add_line();

        // Add data type references
        {
            let mut property_includes: Vec<String> = self
                .properties
                .iter()
                .filter(|prop| prop.data_type.is_some())
                .map(|prop| prop.data_type.as_ref().unwrap().get_rust_usage())
                .collect();

            property_includes.dedup();
            property_includes.sort();

            for include in property_includes {
                generator.append(include).add_line();
            }
        }

        generator
            .add_line()
            .append(format!("pub struct {} ", self.rust_struct_name()))
            .append("{".to_string());

        if self.properties.is_empty() == false {
            generator.indent();

            for prop in self.properties.iter() {
                generator.add_line().append(format!(
                    "pub {}: {},",
                    prop.identifier,
                    prop.data_type.as_ref().unwrap().rust_struct_name()
                ));
            }

            generator.unindent().add_line();
        }

        return generator
            .append("}".to_string())
            .add_lines(2)
            .append("impl Component for ".to_string())
            .append(self.rust_struct_name())
            .append(" {".to_string())
            .indent()
            .add_line()
            .append("type Storage = VecStorage<Self>;".to_string())
            .unindent()
            .add_line()
            .append("}".to_string())
            .to_string();
    }

    fn rust_struct_name(&self) -> String {
        return format!("{}Component", self.identifier().to_title_case());
    }

    fn get_rust_usage(&self) -> String {
        return format!(
            "use crate::{}::components::{}::{};",
            self.root_crate(),
            self.identifier(),
            self.rust_struct_name()
        );
    }

    fn compile(data: &SpecterData) {
        let path = format!("{}/components", data.base_path());

        fs::create_dir_all(path.clone()).unwrap();

        for component in &data.components {
            let component_path = format!("{}/{}.rs", path, component.identifier());

            let mut f = fs::File::create(component_path).unwrap();
            let mut file = LineWriter::new(f);

            let mut generator = StringGenerator::new();
            generator
                .append(SpecterData::code_header())
                .add_line()
                .append(component.to_rust_definition());

            file.write_all(generator.to_string().as_bytes()).unwrap();

            file.flush().unwrap();
        }

        let crates = data.components.iter().map(|obj| obj.identifier()).collect();

        // Create world linker
        let mut generator = StringGenerator::new();

        for component in &data.components {
            generator.append(component.get_rust_usage()).add_line();
        }

        generator
            .add_line()
            .append(Component::get_library_includes())
            .add_lines(2)
            .append("pub fn world_linker(world: &mut specs::World) {".to_string())
            .indent();

        for component in &data.components {
            generator.add_line().append(format!(
                "world.register::<{}>();",
                component.rust_struct_name()
            ));
        }

        generator.unindent().add_line().append("}".to_string());

        SpecterData::compile_module(path, crates, Some(generator.to_string()));
    }
}

impl Identifiable for Component {
    fn identifier(&self) -> String {
        return self.identifier.clone();
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub identifier: String,
    pub data_type: Option<DataType>,
    pub prop_type: String,
}

impl Property {
    pub fn new(identifier: String, prop_type: String) -> Self {
        return Self {
            identifier: identifier.to_lowercase(),
            prop_type: prop_type.to_lowercase(),
            data_type: None,
        };
    }
}

pub fn parse_component(inner_pair: pest::iterators::Pair<'_, Rule>) -> Component {
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

        let prop_type = inner.as_span().as_str().to_string();

        let property = Property::new(prop_identifier.unwrap().to_string(), prop_type);

        properties.push(property);
        prop_identifier = None; // Reset the identifiers
    }

    let component_identity = component_identity.unwrap().to_lowercase();

    return Component::new(component_identity, properties);
}
