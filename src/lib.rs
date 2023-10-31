// Copyright 2023-2023 the slutils-rs authors.

#![allow(dead_code, clippy::needless_return)]
#![forbid(unsafe_code)]
#![warn(
    // TODO: frequently check
    // unreachable_pub,
    // TODO: Activate if you're feeling like fixing stuff
    clippy::pedantic,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    // TODO: Activate when docs are more complete
    // missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications,
    clippy::nursery,
    bad_style,
    // dead_code,
    improper_ctypes,
    missing_copy_implementations,
    missing_debug_implementations,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    trivial_numeric_casts,
    unused_results,
    trivial_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unconditional_recursion,
    // unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    clippy::cast_lossless,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::manual_string_new,
    clippy::match_same_arms,
    clippy::semicolon_if_nothing_returned,
    clippy::trivially_copy_pass_by_ref
)]

mod slp;
mod util;

use std::path::PathBuf;

use crate::slp::slp::parse_slp;

pub fn parse_file(path: PathBuf) {
	let content = std::fs::read(path).expect("Failed to read file");
	let slp = parse_slp(content);

	println!("{}", slp.header);
	println!("---------------------");
	for frame_info in slp.frame_infos {
		println!("{frame_info}");
		println!("---------------------");
	}
	for frame_data in slp.frames {
		println!("{frame_data}");
		println!("---------------------");
	}
}
