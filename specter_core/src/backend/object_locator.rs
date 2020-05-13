use walkdir::{DirEntry, WalkDir};

/// Traverse through the directories locate relevant files
pub fn locate_objects(file_type: &'static str) -> Vec<String> {
    let mut objects_found = vec![];

    for entry in WalkDir::new("./")
        .into_iter()
        .filter_entry(|e| !is_target_dir(e))
    {
        let entry = entry.unwrap();

        // Source all relevant objects
        let obj = from_entry(&entry, file_type);

        if obj.is_some() {
            objects_found.push(obj.unwrap());
        }
    }

    return objects_found;
}

fn from_entry(entry: &DirEntry, file_type: &'static str) -> Option<String> {
    if is_type(&entry, file_type) {
        let path_str = entry.path().to_str();

        if path_str.is_some() {
            let path_str = String::from(path_str.unwrap());

            return Some(path_str);
        }
    }

    return None;
}

fn is_target_dir(entry: &DirEntry) -> bool {
    return entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("target"))
        .unwrap_or(false);
}

fn is_type(entry: &DirEntry, s: &str) -> bool {
    let e = entry.path().to_str();

    if e.is_none() {
        return false;
    }

    let e = e.unwrap();

    return e.ends_with(s);
}
