use super::object_locator::{ObjectTypes, SpecterFileObject};
use inflector::Inflector;

#[derive(Debug)]
pub struct System {
    pub name: String,
}

impl System {
    pub fn rust_name(&self) -> String {
        return format!("{}System", self.name.to_title_case());
    }

    pub fn rust_module(&self) -> String {
        return format!("{}_system", self.name);
    }

    pub fn to_rust_definition(&self) -> String {
        let mut s = format!("pub struct {} {{", self.rust_name());

        let s = format!("{}\n}}", s);

        return s;
    }
}

pub fn build(objects: &Vec<SpecterFileObject>) -> Vec<System> {
    return vec![];
}
