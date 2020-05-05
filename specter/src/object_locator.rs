use walkdir::{DirEntry, WalkDir};

/// Traverse through the directories and build the relevant specter files
pub fn locate_objects() -> Vec<SpecterFileObject> {
    let mut objects_found = vec![];

    for entry in WalkDir::new("./")
        .into_iter()
        .filter_entry(|e| !is_target_dir(e))
    {
        let entry = entry.unwrap();

        // Source all relevant objects
        let obj = SpecterFileObject::from_entry(&entry);

        if obj.is_some() {
            objects_found.push(obj.unwrap());
        }
    }

    return objects_found;
}

#[derive(Debug, PartialEq)]
pub enum ObjectTypes {
    ComponentsDefinition,
    System,
}

impl ObjectTypes {
    fn file_type(&self) -> &str {
        return match self {
            ObjectTypes::ComponentsDefinition => ".cmps",
            ObjectTypes::System => ".sys",
        };
    }
}

#[derive(Debug)]
pub struct SpecterFileObject {
    pub kind: ObjectTypes,
    pub path: Option<String>,
}

impl SpecterFileObject {
    pub fn from_entry(entry: &DirEntry) -> Option<Self> {
        if is_type(&entry, ".specter") {
            let path_str = entry.path().to_str();

            if path_str.is_some() {
                let path_str = String::from(path_str.unwrap());

                let obj = SpecterFileObject {
                    kind: ObjectTypes::ComponentsDefinition,
                    path: Some(path_str),
                };

                return Some(obj);
            }
        }

        return None;
    }
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
