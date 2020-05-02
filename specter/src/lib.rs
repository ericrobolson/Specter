mod object_locator;

/// Build the Specter files
pub fn build() {
    let objects_found = object_locator::locate_objects();

    for obj in objects_found {
        println!("{:?}", obj);
    }
}
