use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

use super::build_systems::System;
use super::file_sys::*;

pub fn generate(systems: &Vec<System>) {
    fs::create_dir_all(format!("{}", SYSTEM_DIRECTORY())).unwrap();

    /*
    for component in components {
        gen_component(component);
    }
    */

    gen_module(systems);
}
/*
fn gen_component(component: &Component) {
    let f = File::create(format!(
        "{}/{}_component.rs",
        COMPONENT_DIRECTORY(),
        component.name
    ))
    .unwrap();

    let mut file = LineWriter::new(f);

    init_file(&mut file);

    file.write_all(component.to_rust_definition().as_bytes())
        .unwrap();

    file.flush().unwrap();
}
*/
fn gen_module(systems: &Vec<System>) {
    let f = File::create(format!("{}/mod.rs", SYSTEM_DIRECTORY())).unwrap();
    let mut file = LineWriter::new(f);

    init_file(&mut file);

    // Write includes/modules
    for system in systems {
        let module_include = format!("\npub mod {};", system.rust_module());
        file.write_all(module_include.as_bytes()).unwrap();

        let module_use = format!("\nuse {}::{};\n", system.rust_module(), system.rust_name());
        file.write_all(module_use.as_bytes()).unwrap();
    }

    file.flush().unwrap();
}