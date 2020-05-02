mod build_components;
mod file_sys;
mod generate_components;
mod generate_module;
mod object_locator;

use std::fs;

/// Build the Specter files
pub fn build() {
    fs::remove_dir_all("src/specter/").unwrap(); // TODO: look into only changing updated files?

    let objects_found = object_locator::locate_objects();

    let components = build_components::build(&objects_found);

    generate_components::generate(&components);

    generate_module::gen_module();
}
