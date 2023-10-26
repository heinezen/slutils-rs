// Copyright 2023-2023 the slutils-rs authors.

mod slp;

use std::path::PathBuf;

use crate::slp::slp::parse_slp;

pub fn parse_file(path: PathBuf) {
    let content = std::fs::read(path).expect("Failed to read file");
    let slp = parse_slp(content);

    println!("{}", slp.header.version);
}
