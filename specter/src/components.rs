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
}

impl Rustable for Component {
    fn to_rust_definition(&self) -> String {
        let mut generator = StringGenerator::new();

        // Add library references
        generator.append(self.get_library_includes()).add_lines(2);

        generator
            .append(format!("pub struct {} ", self.rust_struct_name()))
            .append("{".to_string());

        if self.properties.is_empty() == false {
            generator.indent();

            for prop in self.properties.iter() {
                generator
                    .add_line()
                    .append(format!("pub {}: {:?},", prop.identifier, prop.prop_type));
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
}

impl Identifiable for Component {
    fn identifier(&self) -> String {
        return self.identifier.clone();
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub identifier: String,
    pub default_value: String,
    pub prop_type: Rule,
}

impl Property {
    pub fn new(identifier: String, default_value: String, prop_type: Rule) -> Self {
        return Self {
            identifier: identifier.to_lowercase(),
            default_value: default_value,
            prop_type: prop_type,
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

pub fn compile(data: &SpecterData) {
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
    SpecterData::compile_module(path, crates);
}
