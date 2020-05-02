use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

use super::build_components::{Component, Property};
use super::file_sys::*;

const COMPONENT_FILE_PATH: &'static str = "src/specter/components";

pub fn generate(components: &Vec<Component>) {
    fs::create_dir_all(format!("{}", COMPONENT_FILE_PATH)).unwrap();

    for component in components {
        gen_component(component);
    }

    gen_module(components);
}

fn gen_component(component: &Component) {
    let f = File::create(format!(
        "{}/{}_component.rs",
        COMPONENT_FILE_PATH, component.name
    ))
    .unwrap();

    let mut file = LineWriter::new(f);

    prepend_header(&mut file);

    file.write_all(component.to_rust().as_bytes()).unwrap();

    println!("{:?}", component);

    file.flush().unwrap();
}

fn gen_module(components: &Vec<Component>) {
    let f = File::create(format!("{}/mod.rs", COMPONENT_FILE_PATH)).unwrap();
    let mut file = LineWriter::new(f);

    prepend_header(&mut file);

    for component in components {
        let module_include = format!("pub mod {}_component;\n", component.name);
        file.write_all(module_include.as_bytes()).unwrap();
    }

    file.flush().unwrap();
}
