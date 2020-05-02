mod build_components;
mod object_locator;

/// Build the Specter files
pub fn build() {
    let objects_found = object_locator::locate_objects();

    build_components::build(&objects_found);
}
