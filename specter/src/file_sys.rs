use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, LineWriter};
use std::path::Path;

pub fn prepend_header(writer: &mut std::io::LineWriter<std::fs::File>) {
    writer
        .write_all(b"//THIS IS A GENERATED FILE AND SHOULD NOT BE MODIFIED BY HAND\n")
        .unwrap();
}
