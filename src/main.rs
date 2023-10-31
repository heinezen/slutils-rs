// Copyright 2023-2023 the slutils-rs authors.

use clap::Parser;
use slutils::parse_file;

/// Command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// The path to the file to read
	path: std::path::PathBuf,
}

fn main() {
	let args = Cli::parse();

	parse_file(args.path);
}
