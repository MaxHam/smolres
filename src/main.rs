mod cli;
mod decoder;
mod encoder;
mod transformer;

use clap::Parser;
use cli::Args;
use decoder::decode;
use encoder::encode;
use transformer::{down_sample, up_sample};

fn main() {
    let args = Args::parse();

    println!("Settings");
    println!("Input path: {}", args.input.display());
    println!("Output path: {}", args.output.display());
    println!("Virtual resolution: {}", args.resolution);
    println!("Bit depth: {}", args.bit_depth);

    // Decode image file
    let (pixel_vec, metadata) = decode(&args.input);
    println!("Metadata {:?}", metadata);
    // Transform
    let downsampled_pixel_vec: Vec<u8> = down_sample(
        pixel_vec,
        metadata.width.into(),
        metadata.height.into(),
        args.resolution.into(),
        args.resolution.into(),
        metadata.pixel_format,
    );

    let upsampled_pixel_vec: Vec<u8> = up_sample(
        downsampled_pixel_vec,
        args.resolution.into(),
        args.resolution.into(),
        metadata.height.into(),
        metadata.width.into(),
        metadata.pixel_format,
    );

    // encode to image file
    encode(
        upsampled_pixel_vec,
        metadata.height,
        metadata.width,
        &args.output,
    );
}
