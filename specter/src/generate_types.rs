use super::file_sys::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

pub fn generate() {
    fs::create_dir_all(format!("{}", TYPES_DIRECTORY())).unwrap();

    let f = File::create(format!("{}/mod.rs", TYPES_DIRECTORY())).unwrap();
    let mut file = LineWriter::new(f);

    prepend_header(&mut file);

    gen_type("Vec2", &mut file);
    gen_type("Vec3", &mut file);
    gen_type("Number", &mut file);

    file.flush().unwrap();
}

fn gen_type(type_name: &str, writer: &mut std::io::LineWriter<std::fs::File>) {
    let mut s = format!("pub struct {} {{", type_name);
    let s = format!("{}\n}}", s);

    writer.write_all(s.as_bytes()).unwrap();
}
