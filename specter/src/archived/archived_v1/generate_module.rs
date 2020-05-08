use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

use super::file_sys::*;

use super::build_components::{Component, Property};

pub fn gen_module() {
    let f = File::create(format!("{}/mod.rs", BASE_DIRECTORY())).unwrap();
    let mut file = LineWriter::new(f);

    prepend_header(&mut file);

    file.write_all(b"pub mod components;\n").unwrap();
    file.write_all(b"pub mod systems;\n").unwrap();
    file.write_all(b"pub mod types;\n").unwrap();

    file.flush().unwrap();
}
