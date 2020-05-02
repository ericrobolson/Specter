use super::object_locator::{ObjectTypes, SpecterFileObject};

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

const COMPONENT_PREFIX: &'static str = "component-";

const COMPONENT_FILE_PATH: &'static str = "src/specter/components";

pub fn build(objects: &Vec<SpecterFileObject>) {
    fs::create_dir_all(format!("{}", COMPONENT_FILE_PATH)).unwrap();

    let objects = objects.iter().filter(|o| {
        if o.kind == ObjectTypes::Component {
            if o.path.is_none() {
                panic!("Path not attached for component!");
            }

            return true;
        }

        return false;
    });

    for object in objects {
        let path_str = object.path.as_ref().unwrap();

        if let Ok(lines) = read_lines(path_str) {
            let mut component_name;
            for (i, line) in lines.enumerate() {
                if let Ok(s) = line {
                    let first_line = i == 0;

                    if first_line {
                        component_name = s.replace(COMPONENT_PREFIX, "").replace(":", "");

                        let f = File::create(format!(
                            "{}/{}_component.rs",
                            COMPONENT_FILE_PATH, component_name
                        ))
                        .unwrap();

                        let mut file = LineWriter::new(f);
                        file.write_all(
                            b"//THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND",
                        )
                        .unwrap();
                        file.flush().unwrap();
                    }

                    println!("{}: {}", i, s);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
