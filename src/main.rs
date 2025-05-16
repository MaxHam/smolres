mod cli;
mod decoder;
mod encoder;

use clap::Parser;
use cli::Args;
use decoder::decode;
use encoder::encode;

fn main() {
    let args = Args::parse();

    println!("Settings");
    println!("Input path: {}", args.input.display());
    println!("Output path: {}", args.output.display());
    println!("Virtual resolution: {}", args.resolution);
    println!("Bit depth: {}", args.bit_depth);

    // Decode image file
    let (pixel_vec, metadata) = decode(&args.input);
    // Transform
    let transformed_pixel_vec: Vec<u8> = pixel_vec;
    // encode to image file
    encode(transformed_pixel_vec, metadata, &args.output);
}
