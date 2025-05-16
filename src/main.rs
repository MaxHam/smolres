mod cli;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    println!("Settings");
    println!("Input path: {}", args.input.display());
    println!("Output path: {}", args.output.display());
    println!("Virtual resolution: {}", args.resolution);
    println!("Bit depth: {}", args.bit_depth);
}
