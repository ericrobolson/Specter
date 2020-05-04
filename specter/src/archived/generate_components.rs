use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

use super::build_components::{Component, Property};
use super::file_sys::*;

pub fn generate(components: &Vec<Component>) {
    fs::create_dir_all(format!("{}", COMPONENT_DIRECTORY())).unwrap();

    for component in components {
        gen_component(component);
    }

    gen_module(components);
}

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

fn gen_module(components: &Vec<Component>) {
    let f = File::create(format!("{}/mod.rs", COMPONENT_DIRECTORY())).unwrap();
    let mut file = LineWriter::new(f);

    init_file(&mut file);

    // Write includes/modules
    for component in components {
        let module_include = format!("\npub mod {};", component.rust_module());
        file.write_all(module_include.as_bytes()).unwrap();

        let module_use = format!(
            "\nuse {}::{};\n",
            component.rust_module(),
            component.rust_name()
        );
        file.write_all(module_use.as_bytes()).unwrap();
    }

    // Write world linker
    {
        file.write_all(b"\npub fn link_components(world: &mut specs::World) {")
            .unwrap();

        for component in components {
            let link = format!("\n\tworld.register::<{}>();", component.rust_name());
            file.write_all(link.as_bytes()).unwrap();
        }

        file.write_all(b"\n}").unwrap();
    }

    file.flush().unwrap();
}
