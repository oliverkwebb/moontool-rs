use clap::Parser;

/// A program to print formatted information about the moon
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Formatting Strings
	#[arg(short, long)]
	format: String,
}

fn main() {
    println!("Hello, world!");
}
