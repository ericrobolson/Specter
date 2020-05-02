mod build_components;
mod build_systems;
mod file_sys;
mod generate_components;
mod generate_module;
mod generate_systems;
mod generate_types;
mod object_locator;
use std::fs;
use std::path::Path;

/// Build the Specter files
pub fn build() {
    if Path::new(&file_sys::BASE_DIRECTORY()).exists() {
        fs::remove_dir_all(file_sys::BASE_DIRECTORY()).unwrap(); // TODO: look into only changing updated files?
    }

    let objects_found = object_locator::locate_objects();

    let components = build_components::build(&objects_found);
    let systems = build_systems::build(&objects_found, &components);

    generate_types::generate();

    generate_components::generate(&components);
    generate_systems::generate(&systems);

    generate_module::gen_module();
}
