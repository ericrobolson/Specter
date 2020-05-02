use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

pub fn BASE_DIRECTORY() -> String {
    return String::from("src/specter_gen");
}

pub fn COMPONENT_DIRECTORY() -> String {
    return format!("{}/components", BASE_DIRECTORY());
}

pub fn SYSTEM_DIRECTORY() -> String {
    return format!("{}/systems", BASE_DIRECTORY());
}

pub fn TYPES_DIRECTORY() -> String {
    return format!("{}/types", BASE_DIRECTORY());
}

pub fn init_file(writer: &mut std::io::LineWriter<std::fs::File>) {
    prepend_header(writer);
    add_includes(writer);
}

pub fn prepend_header(writer: &mut std::io::LineWriter<std::fs::File>) {
    writer
        .write_all(b"//THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND\n")
        .unwrap();
}

fn add_includes(writer: &mut std::io::LineWriter<std::fs::File>) {
    writer.write_all(b"use specs::prelude::*;\n").unwrap();
    writer
        .write_all(b"use crate::specter_gen::types::*;\n")
        .unwrap();
}
