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

fn box_str(s: &str) -> String {
    let s = format!("// {} //", s);

    let mut padding = String::new();
    for _ in s.chars() {
        padding.push('/');
    }

    return format!("{}\n{}\n{}\n", padding, s, padding);
}

fn center(chars_in_line: usize, s: &str) -> String {
    let mut s = String::from(s);

    let pad_len = chars_in_line / 4;

    for i in 0..pad_len {
        s.insert(0, ' ');
        s.push(' ');
    }

    return s;
}

pub fn prepend_header(writer: &mut std::io::LineWriter<std::fs::File>) {
    let warning = "THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND - Specter v0.0.1";

    writer.write_all(box_str(warning).as_bytes()).unwrap();
}

fn add_includes(writer: &mut std::io::LineWriter<std::fs::File>) {
    writer.write_all(b"use specs::prelude::*;\n").unwrap();
    writer
        .write_all(b"use crate::specter_gen::types::*;\n")
        .unwrap();
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
