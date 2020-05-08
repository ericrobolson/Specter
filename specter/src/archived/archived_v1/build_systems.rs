use super::object_locator::{ObjectTypes, SpecterFileObject};
use inflector::Inflector;

use super::file_sys::*;

use super::build_components::Component;

#[derive(Debug)]
pub struct SystemComponent {
    pub writeable: bool,
    pub component: Component,
}
#[derive(Debug)]
struct SystemExecuteStatement {
    pub statement_lines: Vec<String>,
    pub aliases: Vec<String>,
}

#[derive(Debug)]
pub struct System {
    pub name: String,
    pub components: Vec<SystemComponent>,
}

impl System {
    pub fn rust_name(&self) -> String {
        return format!("{}System", self.name.to_title_case());
    }

    pub fn rust_module(&self) -> String {
        return format!("{}_system", self.name);
    }

    pub fn rust_dependencies(&self) -> String {
        let mut deps = String::new();

        for component in self.components.iter() {
            deps = format!(
                "{}\nuse crate::specter_gen::components::{}::{};",
                deps,
                component.component.rust_module(),
                component.component.rust_name()
            );
        }

        return deps;
    }

    fn rust_system_data(&self) -> String {
        let mut component_defs = String::new();

        for component in self.components.iter() {
            let storage_type = {
                if component.writeable {
                    "\t\tWriteStorage"
                } else {
                    "\t\tReadStorage"
                }
            };

            let def = format!("{}<'a, {}>,", storage_type, component.component.rust_name());

            component_defs = format!("{}\n{}", component_defs, def);
        }

        return format!("type SystemData = ({}\n\t);", component_defs);
    }

    fn rust_fn_run(&self) -> String {
        let types = {
            let mut val = String::new();

            let components_len = self.components.len() - 1;
            for (i, component) in self.components.iter().enumerate() {
                if component.writeable {
                    val = format!("{}mut ", val);
                }

                // Write pluralized name
                val = format!("{}{}s", val, component.component.name);

                // Append comma if not the last one
                if i != components_len {
                    val = format!("{}, ", val);
                }
            }

            val
        };

        let implementation = "";

        return format!(
            "fn run(&mut self, ({}): Self::SystemData) {{ \n{}\n \t}}",
            types, implementation
        );
    }

    pub fn to_rust_definition(&self) -> String {
        let definition = format!("\n\npub struct {};", self.rust_name());

        let inner_implementation = format!(
            "{{\n\t{}\n\n\t{}\n }}",
            self.rust_system_data(),
            self.rust_fn_run()
        );
        let implementation = format!(
            "impl<'a> System<'a> for {} {}",
            self.rust_name(),
            inner_implementation
        );

        return format!("{}\n\n{}", definition, implementation);
    }
}

pub fn build(objects: &Vec<SpecterFileObject>, components: &Vec<Component>) -> Vec<System> {
    let objects = objects.iter().filter(|o| {
        if o.kind == ObjectTypes::System {
            if o.path.is_none() {
                panic!("Path not attached for system!");
            }

            return true;
        }
        return false;
    });

    let mut systems = vec![];

    for object in objects {
        let path_str = object.path.as_ref().unwrap();

        if let Ok(lines) = read_lines(path_str) {
            let mut name = None;
            let mut first_line = true;
            let mut sys_components: Vec<SystemComponent> = vec![];

            let mut execute_statement: Option<SystemExecuteStatement> = None;
            let mut execute_statements: Vec<SystemExecuteStatement> = vec![];

            for line in lines {
                if let Ok(mut s) = line {
                    s.retain(|c| !c.is_whitespace());
                    if s.is_empty() {
                        continue;
                    }

                    if first_line {
                        // Get the name
                        name = Some(s.replace(":", ""));
                        first_line = false;
                        continue;
                    }

                    // Link up the components found
                    if s.starts_with('(') {
                        let component_join: Vec<&str> = s.split(',').collect();

                        for component_def in component_join {
                            let split_component: Vec<&str> = component_def.split(':').collect();
                            let alias = split_component[0];
                            let component_type = split_component[1];

                            let component_type_def: Vec<&str> = component_type.split('.').collect();

                            let writeable = component_type_def[0] == "c-w";
                            let component_name = component_type_def[1]
                                .to_string()
                                .replace(')', "")
                                .replace('{', "")
                                .replace('}', "");

                            // Check if it already exists
                            let mut exists = false;
                            {
                                for mut component in sys_components.iter_mut() {
                                    if component.component.name == component_name {
                                        exists = true;

                                        if writeable && !component.writeable {
                                            component.writeable = true;
                                        }
                                    }
                                }
                            }

                            // If not, push it in
                            if !exists {
                                let matched_component =
                                    components.iter().find(|c| c.name == component_name);

                                if matched_component.is_none() {
                                    let names: Vec<String> =
                                        components.iter().map(|c| c.name.clone()).collect();

                                    panic!("Unable to match component with name: '{}'. Available components: {:?}", component_name, names);
                                }

                                let matched_component = matched_component.unwrap();

                                let component = SystemComponent {
                                    writeable: writeable,
                                    component: matched_component.clone(),
                                };

                                sys_components.push(component);
                            }
                        }
                    }
                }
            }

            if name.is_some() {
                sys_components
                    .sort_by(|a, b| a.component.name.partial_cmp(&b.component.name).unwrap());

                let system = System {
                    name: name.unwrap().to_lowercase(),
                    components: sys_components,
                };

                systems.push(system);
            }
        }
    }

    return systems;
}
